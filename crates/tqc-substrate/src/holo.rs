//! Execution of operations as native `.holo` artifacts.

use hologram_backend::CpuBackend;
use hologram_compiler::{compile, BackendKind};
use hologram_exec::{buffer::InputBuffer, BufferArena, InferenceSession};
use hologram_graph::constant::ConstantEntry;
use hologram_graph::graph::Graph;
use hologram_graph::node::{GatherAttrs, GraphOp, InputSource, Node};
use hologram_graph::registry::DTypeId;
use hologram_ops::OpKind;
use uor_foundation::WittLevel;

/// Compiles a permutation gate into a `.holo` artifact and executes it on the native engine.
///
/// This dynamically constructs a computation graph with a `Gather` op, compiles it to an archive,
/// and runs it using `InferenceSession` over the binary-encoded κ-state inputs.
pub fn execute_holo_gate(targets: &[usize], state_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut g = Graph::new();
    let dtype_i64 = DTypeId(5); // DTYPE_I64 is 5
    let input_len = (state_bytes.len() / 8) as u64;
    let shape_input = g
        .shape_registry_mut()
        .intern(hologram_graph::registry::ShapeDescriptor::rank1(input_len));
    let shape_indices =
        g.shape_registry_mut()
            .intern(hologram_graph::registry::ShapeDescriptor::rank1(
                targets.len() as u64 * 2,
            ));

    let in_node = g.add_node(Node {
        op: GraphOp::Input,
        inputs: smallvec::smallvec![],
        output_dtype: dtype_i64,
        output_shape: shape_input,
    });
    g.add_named_input(in_node, "state");

    let mut indices = Vec::with_capacity(targets.len() * 2);
    for &t in targets {
        indices.push((t * 2) as i64);
        indices.push((t * 2 + 1) as i64);
    }
    let indices_bytes: Vec<u8> = indices.iter().flat_map(|&x| x.to_le_bytes()).collect();

    let cid = g.constants_mut().insert(ConstantEntry {
        bytes: indices_bytes,
        dtype: dtype_i64,
        shape: shape_indices,
    });

    let c_node = g.add_node(Node {
        op: GraphOp::Constant(cid),
        inputs: smallvec::smallvec![],
        output_dtype: dtype_i64,
        output_shape: shape_indices,
    });

    let gather_node = g.add_node(Node {
        op: GraphOp::Op(OpKind::Gather),
        inputs: smallvec::smallvec![InputSource::Node(in_node), InputSource::Node(c_node)],
        output_dtype: dtype_i64,
        output_shape: shape_indices,
    });
    g.set_gather_attrs(gather_node, GatherAttrs { axis: 0 });
    g.add_named_output(gather_node, "output");

    let compiled = compile(g, BackendKind::Cpu, WittLevel::W32).map_err(|e| format!("{:?}", e))?;
    let backend = CpuBackend::<BufferArena>::new();
    let mut session =
        InferenceSession::load(&compiled.archive, backend).map_err(|e| format!("{:?}", e))?;

    let outputs = session
        .execute(&[InputBuffer { bytes: state_bytes }])
        .map_err(|e| format!("{:?}", e))?;
    Ok(outputs[0].bytes.to_vec())
}
