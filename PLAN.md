# Orchestration App — Implementation Plan

## Context

Build a fast, native desktop app for orchestrating CLI-based coding agents (Claude Code, Aider, Codex, etc.). The app provides a structured chat UI (not a raw terminal) with a text input, toolbar controls (worktree selector, model selector, agent picker), and rendered markdown output. Agents run as long-lived PTY subprocesses under the hood — the app writes user prompts to stdin and captures/renders stdout as markdown. Linux only. Built from scratch in Rust using GPUI.

**Why this exists**: Existing tools (VS Code extensions, Electron-based wrappers) are slow and clunky. This app should feel as fast as Zed or Alacritty — sub-millisecond input latency, instant panel switching, keyboard-driven workflow.

**Reference projects to study** (not fork): [Arbor](https://github.com/penso/arbor), Zed's `crates/terminal/` and `crates/markdown/`, [gpui-component](https://github.com/longbridge/gpui-component).

---

## Architecture

### GUI Framework: GPUI

GPUI (Zed's framework) wins because:
- Battle-tested in Zed for exactly this kind of app (multi-pane, rich text, diffs)
- `gpui-component` provides 60+ widgets including dock layout, tabs, buttons, text inputs
- GPU-accelerated text rendering, sub-ms input latency
- Linux support is solid
- Zed already has markdown rendering infrastructure we can reference

Trade-off: requires Rust nightly, pre-1.0 with breaking changes, thinner docs than egui.

### Cargo Workspace

```
orchestration-app/
  Cargo.toml                    # workspace root
  rust-toolchain.toml           # pin nightly
  crates/
    hive-core/                  # headless: agent, session, worktree, diff
      src/
        lib.rs
        agent.rs                # Agent subprocess management (PTY spawn, stdin/stdout)
        session.rs              # Session model (chat = session)
        worktree.rs             # git2-based worktree management
        diff.rs                 # Session-scoped diff computation
        config.rs               # App config (TOML)
    hive-gui/                   # GPUI application
      src/
        main.rs                 # entry point
        app.rs                  # top-level app state
        workspace.rs            # dock-based layout
        sidebar.rs              # session list + worktree list
        chat_view.rs            # message history (rendered markdown) + input toolbar
        input_bar.rs            # multi-line text input + toolbar (worktree, model, agent selectors)
        message.rs              # individual message rendering (markdown + code blocks)
        diff_panel.rs           # per-session diff viewer
        command_palette.rs      # fuzzy command search (Ctrl+K)
        theme.rs
        keybindings.rs
```

Note: `hive-terminal` crate removed — no embedded terminal. Agent communication goes through PTY subprocess but output is captured and rendered as markdown, not shown in a terminal emulator.

### Core Abstractions

**Agent** (`hive-core/agent.rs`): Spawns a CLI agent as a long-lived PTY subprocess. Writes user prompts to stdin, reads stdout/stderr. Strips ANSI escape codes from output. Each agent has a config: command, default args, env vars, working directory.

```rust
pub struct AgentConfig {
    pub name: String,           // display name: "Claude Code", "Aider", etc.
    pub command: String,        // "claude", "aider", "codex"
    pub args: Vec<String>,      // default args (e.g., ["--no-color"] if needed)
    pub env: HashMap<String, String>,
}

pub struct AgentProcess {
    pty_writer: Box<dyn Write + Send>,  // write prompts to stdin
    output_rx: mpsc::Receiver<String>,  // receive output chunks
    pid: u32,
    status: AgentStatus,
}
```

**Communication model**: Agent runs as a long-lived interactive process (REPL-style). User types a message → app writes it to the PTY stdin + newline. Agent responds on stdout → app collects output, strips ANSI, and appends as a new message to the chat history. Output is chunked/streamed so rendering updates incrementally.

**Session** (`hive-core/session.rs`): A session = one agent process + chat history + working directory + optional worktree + baseline commit.

```rust
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub agent_config: AgentConfig,
    pub worktree: Option<WorktreeRef>,
    pub working_dir: PathBuf,
    pub messages: Vec<Message>,         // chat history
    pub baseline_commit: Option<String>, // for diff computation
    pub status: SessionStatus,
}

pub struct Message {
    pub role: MessageRole,  // User or Agent
    pub content: String,    // markdown text
    pub timestamp: DateTime<Utc>,
}
```

**WorktreeManager** (`hive-core/worktree.rs`): Uses `git2` crate. Create/list/delete worktrees. User controls when worktrees are created — app provides shortcuts and UI.

**Diff** (`hive-core/diff.rs`): On session start, record HEAD as baseline. Diff panel shows `git diff <baseline>..working_tree` using `git2`. File watcher (`notify` crate) triggers recomputation.

### UI Layout

```
+------------------+------------------------------------------+------------------+
|    SIDEBAR       |              MAIN PANEL                   |    DIFF PANEL    |
|                  |                                          |                  |
|  [Sessions]      |  +------------------------------------+  |  [Changed Files] |
|  - Session 1 *   |  |  Agent message (rendered markdown)  |  |  - src/main.rs   |
|  - Session 2     |  |  ```rust                            |  |  - lib.rs        |
|  - Session 3     |  |  fn main() { ... }                  |  |                  |
|                  |  |  ```                                 |  |  [Unified Diff]  |
|  [Worktrees]     |  |                                    |  |  - old line      |
|  - main          |  |  User message                       |  |  + new line      |
|  - feat/auth     |  |                                    |  |                  |
|  - fix/bug-123   |  |  Agent message (streaming...)       |  |                  |
|                  |  +------------------------------------+  |                  |
|                  |                                          |                  |
|                  |  +------------------------------------+  |                  |
|                  |  | [Worktree ▼] [Model ▼] [Agent ▼]   |  |                  |
|                  |  | Type your message...                |  |                  |
|                  |  |                                     |  |                  |
|                  |  |                          [Send ⏎]   |  |                  |
|                  |  +------------------------------------+  |                  |
+------------------+------------------------------------------+------------------+
|                           STATUS BAR                                           |
+--------------------------------------------------------------------------------+
```

- **Sidebar** (left, collapsible): Session list + worktree list. Click to switch sessions. Keyboard: Ctrl+1..9.
- **Main Panel** (center):
  - **Chat history** (top, scrollable): Messages rendered as styled markdown. User messages right-aligned or visually distinct. Agent messages with code block syntax highlighting.
  - **Input bar** (bottom, fixed): Multi-line text input. Toolbar row above it with dropdown selectors for worktree, model, and agent. Send with Enter (Shift+Enter for newline).
- **Diff Panel** (right, collapsible): Files changed since session baseline. Click file to see unified diff.
- **Status Bar** (bottom): Current worktree, branch, agent status, session duration.

All panels resizable via gpui-component's Dock. Sidebar and diff panel are collapsible.

### Key Dependencies

| Crate | Purpose |
|-------|---------|
| `gpui` | UI framework (pin to specific version) |
| `gpui-component` ~0.5 | Widgets (dock, tabs, buttons, dropdowns, text input) |
| `portable-pty` | PTY spawning for agent subprocesses |
| `git2` ~0.20 | Git operations (diff, worktree, status) |
| `notify` ~7.0 | Filesystem watcher for diff updates |
| `pulldown-cmark` | Markdown parsing for message rendering |
| `syntect` or `tree-sitter-highlight` | Syntax highlighting in code blocks |
| `strip-ansi-escapes` | Strip ANSI from agent output |
| `serde` + `toml` | Configuration |
| `uuid` | Session IDs |
| `chrono` | Timestamps |
| `tokio` ~1.0 | Async runtime |
| `tracing` | Structured logging |

---

## Implementation Phases

### Phase 1: Skeleton
1. Initialize Cargo workspace with two crates (`hive-core`, `hive-gui`)
2. Set up `rust-toolchain.toml` for nightly
3. Get a GPUI window rendering with gpui-component dock layout (three placeholder panels)
4. Wire up basic keyboard shortcuts (Ctrl+B toggle sidebar, Ctrl+D toggle diff panel)

### Phase 2: Chat UI
5. Build the input bar: multi-line text input with send button (Enter to send)
6. Build the chat history view: scrollable list of messages, user vs agent styling
7. Basic markdown rendering for messages (bold, italic, code blocks, headers)
8. Syntax highlighting in code blocks

### Phase 3: Agent Integration
9. Implement agent subprocess spawning via PTY in `hive-core`
10. Wire input bar → agent stdin, agent stdout → chat history
11. Stream agent output incrementally (don't wait for full response)
12. ANSI stripping from agent output before markdown rendering
13. Toolbar: agent selector dropdown (configured agents from config file)

### Phase 4: Sessions
14. Implement session model with chat history persistence
15. Sidebar: session list, create new session, switch between sessions
16. Each session maintains independent agent process and chat history
17. Toolbar: model selector dropdown (passes model flag to agent CLI)

### Phase 5: Worktrees
18. Implement `WorktreeManager` with git2
19. Worktree management in sidebar: create, list, delete
20. Toolbar: worktree selector dropdown — sets working directory for the agent
21. Associate sessions with worktrees

### Phase 6: Diffs
22. Record baseline commit on session start
23. Compute diff using git2 (baseline vs working tree)
24. Diff panel: file list + unified diff view with syntax coloring
25. Auto-refresh diffs on file changes (notify crate)

### Phase 7: Polish
26. Command palette (Ctrl+K) with fuzzy search
27. Config file (`~/.config/hive/config.toml`) for agent definitions, keybindings, theme
28. Session persistence across app restarts (save chat history as JSON/TOML)
29. Keyboard navigation: Ctrl+1..9 session switching, vim-style panel focus

---

## Verification

- **Phase 1**: App launches, shows three-pane dock layout, panels resize, keyboard toggles work
- **Phase 2**: Can type text, see it rendered as a user message, markdown renders correctly with syntax highlighting
- **Phase 3**: Can send a message to a real agent (e.g., `claude`), see streaming response rendered as markdown
- **Phase 4**: Can create multiple sessions, switch between them, each has independent history and agent
- **Phase 5**: Can create/list/delete git worktrees from the UI, select worktree for a session
- **Phase 6**: Start session, agent makes changes, diff panel shows what changed since session started
- **Phase 7**: Command palette works, config file is loaded, sessions survive restart
