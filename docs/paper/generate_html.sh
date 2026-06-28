#!/bin/bash

cat << 'EOF' > public/index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>UOR Atlas Whitepaper</title>
    <meta name="description" content="A Parametric, BDD-Driven, V&V-Gated Realization on Holospaces">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;600;700&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg-color: #0b0f19;
            --text-color: #e2e8f0;
            --accent-color: #3b82f6;
            --glass-bg: rgba(255, 255, 255, 0.03);
            --glass-border: rgba(255, 255, 255, 0.1);
        }
        
        body, html {
            margin: 0;
            padding: 0;
            height: 100%;
            font-family: 'Inter', sans-serif;
            background-color: var(--bg-color);
            color: var(--text-color);
            background-image: 
                radial-gradient(circle at 15% 50%, rgba(59, 130, 246, 0.15), transparent 25%),
                radial-gradient(circle at 85% 30%, rgba(147, 51, 234, 0.15), transparent 25%);
            background-attachment: fixed;
        }

        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
            display: flex;
            flex-direction: column;
            align-items: center;
            min-height: 100vh;
        }

        header {
            text-align: center;
            margin-bottom: 2rem;
            animation: fadeIn 1s ease-out;
        }

        h1 {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 1rem;
            background: linear-gradient(to right, #60a5fa, #c084fc);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }

        p.subtitle {
            font-size: 1.2rem;
            color: #94a3b8;
            max-width: 800px;
            margin: 0 auto 2rem auto;
            line-height: 1.6;
        }

        .glass-panel {
            background: var(--glass-bg);
            backdrop-filter: blur(12px);
            -webkit-backdrop-filter: blur(12px);
            border: 1px solid var(--glass-border);
            border-radius: 16px;
            padding: 2rem;
            width: 100%;
            max-width: 1000px;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
            display: flex;
            flex-direction: column;
            align-items: center;
            flex-grow: 1;
            margin-bottom: 2rem;
            animation: slideUp 0.8s ease-out;
        }
        
        .pdf-container {
            width: 100%;
            flex-grow: 1;
            min-height: 70vh;
            border-radius: 8px;
            overflow: hidden;
            border: 1px solid var(--glass-border);
            background-color: #fff;
        }
        
        iframe {
            width: 100%;
            height: 100%;
            border: none;
        }

        .download-btn {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            background: linear-gradient(135deg, #3b82f6, #2563eb);
            color: white;
            text-decoration: none;
            padding: 1rem 2rem;
            border-radius: 9999px;
            font-weight: 600;
            font-size: 1.1rem;
            transition: all 0.3s ease;
            box-shadow: 0 4px 6px -1px rgba(59, 130, 246, 0.5);
            margin-top: 1.5rem;
        }

        .download-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 15px -3px rgba(59, 130, 246, 0.6);
            background: linear-gradient(135deg, #60a5fa, #3b82f6);
        }

        .download-btn svg {
            width: 24px;
            height: 24px;
        }

        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }

        @keyframes slideUp {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>The UOR Atlas as a Universal Topological Quantum Computer</h1>
            <p class="subtitle">A Parametric, BDD-Driven, V&V-Gated Realization on Holospaces</p>
        </header>

        <div class="glass-panel">
            <div class="pdf-container">
                <iframe src="UOR_Atlas_Whitepaper.pdf" title="UOR Atlas Whitepaper PDF"></iframe>
            </div>
            
            <a href="UOR_Atlas_Whitepaper.pdf" download="UOR_Atlas_Whitepaper.pdf" class="download-btn">
                <svg fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"></path>
                </svg>
                Download PDF
            </a>
        </div>
    </div>
</body>
</html>
EOF
