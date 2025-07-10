fn main() {
    use std::fs;
    use std::collections::HashMap;
    
    // Create a simple HTML visualization manually
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GDK Tree Visualization Demo</title>
    <style>
        body {
            margin: 0;
            padding: 20px;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            overflow: hidden;
        }
        .header {
            background: linear-gradient(135deg, #1e3a8a 0%, #3b82f6 100%);
            color: white;
            padding: 20px 30px;
            text-align: center;
        }
        .header h1 {
            margin: 0;
            font-size: 2rem;
            font-weight: 600;
        }
        .header p {
            margin: 10px 0 0 0;
            opacity: 0.9;
            font-size: 1.1rem;
        }
        .svg-container {
            padding: 20px;
            overflow: auto;
            background: #f8fafc;
        }
        .stats {
            padding: 20px 30px;
            background: #ffffff;
            border-top: 1px solid #e2e8f0;
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
        }
        .stat-card {
            text-align: center;
            padding: 15px;
            background: #f8fafc;
            border-radius: 8px;
            border: 1px solid #e2e8f0;
        }
        .stat-value {
            font-size: 2rem;
            font-weight: bold;
            color: #1e40af;
        }
        .stat-label {
            color: #64748b;
            font-size: 0.9rem;
            margin-top: 5px;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🌳 GDK Tree Visualization</h1>
            <p>Git Workflow Deep Knowledge System - Commit Tree with File Quality Threads</p>
        </div>
        
        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">24</div>
                <div class="stat-label">Total Commits</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">5</div>
                <div class="stat-label">Branches</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">120</div>
                <div class="stat-label">File Threads</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">18</div>
                <div class="stat-label">Converged Commits</div>
            </div>
        </div>
        
        <div class="svg-container">
            <svg width="1200" height="600" xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <style>
                        .node { fill-opacity: 0.8; stroke-width: 2; }
                        .node:hover { fill-opacity: 1.0; stroke-width: 3; }
                        .branch-line { stroke-width: 2; fill: none; }
                        .thread { fill: none; opacity: 0.7; }
                        .thread:hover { opacity: 1.0; }
                        .label { font-family: monospace; font-size: 10px; fill: #333; }
                        .branch-label { font-family: sans-serif; font-size: 12px; font-weight: bold; }
                    </style>
                </defs>
                <rect width="100%" height="100%" fill="#f8fafc"/>
                
                <!-- Main branch line -->
                <line x1="600" y1="50" x2="600" y2="550" stroke="#2E86AB" class="branch-line" opacity="0.3"/>
                <text x="615" y="40" class="branch-label" fill="#2E86AB">main</text>
                
                <!-- Spiral branch lines -->
                <line x1="480" y1="150" x2="480" y2="350" stroke="#A23B72" class="branch-line" opacity="0.3"/>
                <text x="495" y="140" class="branch-label" fill="#A23B72">spiral-0</text>
                
                <line x1="720" y1="200" x2="720" y2="400" stroke="#F18F01" class="branch-line" opacity="0.3"/>
                <text x="735" y="190" class="branch-label" fill="#F18F01">spiral-1</text>
                
                <!-- File quality threads (colorful connecting lines) -->
                <line x1="600" y1="100" x2="600" y2="150" stroke="#22C55E" stroke-width="3" class="thread" opacity="0.6">
                    <title>src/core.rs (Quality: 0.95)</title>
                </line>
                <line x1="602" y1="100" x2="602" y2="150" stroke="#84CC16" stroke-width="2" class="thread" opacity="0.6">
                    <title>src/lib.rs (Quality: 0.82)</title>
                </line>
                <line x1="598" y1="100" x2="598" y2="150" stroke="#EAB308" stroke-width="2" class="thread" opacity="0.6">
                    <title>tests/mod.rs (Quality: 0.67)</title>
                </line>
                
                <line x1="600" y1="150" x2="600" y2="200" stroke="#22C55E" stroke-width="3" class="thread" opacity="0.6">
                    <title>src/core.rs (Quality: 0.97)</title>
                </line>
                <line x1="602" y1="150" x2="602" y2="200" stroke="#22C55E" stroke-width="2" class="thread" opacity="0.6">
                    <title>src/lib.rs (Quality: 0.91)</title>
                </line>
                
                <!-- Spiral threads -->
                <line x1="600" y1="200" x2="480" y2="250" stroke="#F97316" stroke-width="2" class="thread" opacity="0.6">
                    <title>spiral/experiment_0.rs (Quality: 0.43)</title>
                </line>
                <line x1="600" y1="200" x2="720" y2="250" stroke="#EF4444" stroke-width="1" class="thread" opacity="0.6">
                    <title>spiral/experiment_1.rs (Quality: 0.21)</title>
                </line>
                
                <!-- Main branch commit nodes -->
                <circle cx="600" cy="100" r="8" fill="#84CC16" stroke="#2E86AB" class="node">
                    <title>Commit: main-0001 Health: 0.78 Files: 3 Converged: false</title>
                </circle>
                <circle cx="600" cy="100" r="5" fill="#84CC16" opacity="0.8"/>
                <text x="600" y="125" class="label" text-anchor="middle">main-0001</text>
                
                <circle cx="600" cy="150" r="8" fill="#22C55E" stroke="#2E86AB" class="node">
                    <title>Commit: main-0002 Health: 0.89 Files: 3 Converged: true</title>
                </circle>
                <circle cx="600" cy="150" r="5" fill="#22C55E" opacity="0.8"/>
                <circle cx="607" cy="143" r="3" fill="#10B981" stroke="#ffffff" stroke-width="1"/>
                <text x="600" y="175" class="label" text-anchor="middle">main-0002</text>
                
                <circle cx="600" cy="200" r="8" fill="#22C55E" stroke="#2E86AB" class="node">
                    <title>Commit: main-0003 Health: 0.94 Files: 2 Converged: true</title>
                </circle>
                <circle cx="600" cy="200" r="5" fill="#22C55E" opacity="0.8"/>
                <circle cx="607" cy="193" r="3" fill="#10B981" stroke="#ffffff" stroke-width="1"/>
                <text x="600" y="225" class="label" text-anchor="middle">main-0003</text>
                
                <!-- Spiral branch nodes -->
                <circle cx="480" cy="250" r="8" fill="#F97316" stroke="#A23B72" class="node">
                    <title>Commit: spiral-0-0001 Health: 0.43 Files: 2 Converged: false</title>
                </circle>
                <circle cx="480" cy="250" r="5" fill="#F97316" opacity="0.8"/>
                <text x="480" y="275" class="label" text-anchor="middle">spiral-0-0001</text>
                
                <circle cx="720" cy="250" r="8" fill="#EF4444" stroke="#F18F01" class="node">
                    <title>Commit: spiral-1-0001 Health: 0.21 Files: 2 Converged: false</title>
                </circle>
                <circle cx="720" cy="250" r="5" fill="#EF4444" opacity="0.8"/>
                <text x="720" y="275" class="label" text-anchor="middle">spiral-1-0001</text>
                
                <circle cx="480" cy="300" r="8" fill="#EAB308" stroke="#A23B72" class="node">
                    <title>Commit: spiral-0-0002 Health: 0.61 Files: 2 Converged: false</title>
                </circle>
                <circle cx="480" cy="300" r="5" fill="#EAB308" opacity="0.8"/>
                <text x="480" y="325" class="label" text-anchor="middle">spiral-0-0002</text>
                
                <circle cx="720" cy="300" r="8" fill="#F97316" stroke="#F18F01" class="node">
                    <title>Commit: spiral-1-0002 Health: 0.38 Files: 2 Converged: false</title>
                </circle>
                <circle cx="720" cy="300" r="5" fill="#F97316" opacity="0.8"/>
                <text x="720" y="325" class="label" text-anchor="middle">spiral-1-0002</text>
                
                <!-- Legend -->
                <g transform="translate(20, 450)">
                    <rect x="0" y="0" width="200" height="140" fill="white" stroke="#e2e8f0" stroke-width="1" rx="5"/>
                    <text x="10" y="20" class="branch-label" fill="#374151">Thread Quality</text>
                    
                    <circle cx="20" cy="35" r="5" fill="#22C55E"/>
                    <text x="35" y="40" class="label" fill="#374151">Green (0.9+)</text>
                    
                    <circle cx="20" cy="50" r="5" fill="#84CC16"/>
                    <text x="35" y="55" class="label" fill="#374151">Light Green (0.7+)</text>
                    
                    <circle cx="20" cy="65" r="5" fill="#EAB308"/>
                    <text x="35" y="70" class="label" fill="#374151">Yellow (0.5+)</text>
                    
                    <circle cx="20" cy="80" r="5" fill="#F97316"/>
                    <text x="35" y="85" class="label" fill="#374151">Orange (0.3+)</text>
                    
                    <circle cx="20" cy="95" r="5" fill="#EF4444"/>
                    <text x="35" y="100" class="label" fill="#374151">Red (<0.3)</text>
                    
                    <circle cx="20" cy="115" r="3" fill="#10B981" stroke="#ffffff" stroke-width="1"/>
                    <text x="35" y="120" class="label" fill="#374151">Converged</text>
                </g>
            </svg>
        </div>
    </div>
</body>
</html>"#;

    fs::write("gdk_tree_demo.html", html).expect("Failed to write demo file");
    println!("🌳 Demo visualization created: gdk_tree_demo.html");
    println!("   Open this file in your browser to see the interactive commit tree!");
}