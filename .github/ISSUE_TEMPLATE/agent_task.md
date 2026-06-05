name: Agent Task
description: Workstream-level task Epic/Issue for TheNest
title: 'TASK: [Title]'
labels: ['agent-ready']
body:
  - type: markdown
    attributes:
      value: "This issue corresponds to a TASK file in agent/tasks/."
  - type: input
    id: task_id
    attributes:
      label: Task ID
      placeholder: e.g. TASK_P3_01
    validations:
      required: true
