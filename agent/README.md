# Agent Directory

Welcome! This directory structures development work into bounded tasks and microtasks to protect subagent context limits.

## Agent Work Instructions

1. **Pick a Task:** Look at [tasks index](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/README.md) and see what is currently uncompleted in the queue.
2. **Execute Bounded Microtasks:** Go to the [microtasks index](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/README.md) and pick the next sequential microtask for that task.
3. **Minimize Context:**
   - Do not read the whole repo unless blocked.
   - Do not widen scope.
   - If required files are missing, report BLOCKED.
4. **Forbidden Work:**
   - Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.
5. **Report Completion:** Write a short final report in your output and stop. Do not start subsequent tasks without explicit dispatch.
