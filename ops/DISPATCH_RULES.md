# Dispatch Rules

- **Low-Context Bounded Execution:** Subagents must only read the minimal context files specified in their assigned task/microtask.
- **Strict Scope Boundaries:** Subagents are forbidden from modifying files outside the "Allowed edits" list.
- **Single Slice Dispatch:** Assign only one microtask to a subagent at a time.
- **No Architectural Drift:** Reject and report any tasks that imply adding runtimes, schedulers, queues, databases, HTTP servers, Tokio, or network layers. Keep the codebase file-only and local.
- **Lightweight Validation:** Always execute the default workspace validation commands before and after edits.
- **Progress Tracking:** Update completed slices and context cards upon successful microtask verify-and-merge.
- **Done Condition:** Stop immediately and return a 2-line summary report when the stop condition is reached.
