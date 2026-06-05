name: Microtask
description: Bounded, low-context microtask for subagent execution
title: 'MICRO: [Title]'
labels: ['microtask', 'agent-ready']
body:
  - type: markdown
    attributes:
      value: "This issue corresponds to a MICRO file in agent/microtasks/."
  - type: input
    id: micro_id
    attributes:
      label: Microtask ID
      placeholder: e.g. MICRO_P3_01_01
    validations:
      required: true
