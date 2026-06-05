# Microtasks Index

This index defines granular, bounded steps that can be solved by subagents in seconds.

## Rules for Subagents

1. **One subagent per microtask:** Do not execute multiple microtasks in one thread.
2. **Minimal context packet:** Read ONLY the files listed in the context pack.
3. **Allowed edits rule:** Do NOT touch any file not explicitly listed under "Allowed edits".
4. **Stop condition:** Submit your 2-line report and immediately stop.

## Microtask Status Table
| Microtask ID | Parent Task | Title | Status | Link |
|---|---|---|---|---|
| P3_01_01 | P3_01 | REMOVE_RUNTIME_QUEUE_REFERENCES | PLANNED | [MICRO_P3_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_01_01_REMOVE_RUNTIME_QUEUE_REFERENCES.md) |
| P3_01_02 | P3_01 | CREATE_CLI_ARG_SPEC | PLANNED | [MICRO_P3_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_01_02_CREATE_CLI_ARG_SPEC.md) |
| P3_02_01 | P3_02 | DECIDE_DEMO_VS_HIVE_WORKBENCH | PLANNED | [MICRO_P3_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_01_DECIDE_DEMO_VS_HIVE_WORKBENCH.md) |
| P3_02_02 | P3_02 | UPDATE_CARGO_BINARY_TARGET | PLANNED | [MICRO_P3_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_02_UPDATE_CARGO_BINARY_TARGET.md) |
| P3_02_03 | P3_02 | UPDATE_VALIDATE_SCRIPT_BINARY_REFERENCES | PLANNED | [MICRO_P3_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_03_UPDATE_VALIDATE_SCRIPT_BINARY_REFERENCES.md) |
| P3_03_01 | P3_03 | PARSE_COMMAND_NAME | PLANNED | [MICRO_P3_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_01_PARSE_COMMAND_NAME.md) |
| P3_03_02 | P3_03 | PARSE_OUTPUT_FLAG | PLANNED | [MICRO_P3_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_02_PARSE_OUTPUT_FLAG.md) |
| P3_03_03 | P3_03 | STABLE_USAGE_ERRORS | PLANNED | [MICRO_P3_03_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_03_STABLE_USAGE_ERRORS.md) |
| P3_04_01 | P3_04 | LIST_SCENARIOS | PLANNED | [MICRO_P3_04_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_01_LIST_SCENARIOS.md) |
| P3_04_02 | P3_04 | RUN_NAMED_SCENARIO | PLANNED | [MICRO_P3_04_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_02_RUN_NAMED_SCENARIO.md) |
| P3_04_03 | P3_04 | RUN_FILE_SCENARIO | PLANNED | [MICRO_P3_04_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_03_RUN_FILE_SCENARIO.md) |
| P3_04_04 | P3_04 | RUN_SCENARIO_SUITE | PLANNED | [MICRO_P3_04_04](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_04_RUN_SCENARIO_SUITE.md) |
| P3_05_01 | P3_05 | HUMAN_OUTPUT_MODE | PLANNED | [MICRO_P3_05_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_05_01_HUMAN_OUTPUT_MODE.md) |
| P3_05_02 | P3_05 | JSON_OUTPUT_MODE | PLANNED | [MICRO_P3_05_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_05_02_JSON_OUTPUT_MODE.md) |
| P3_05_03 | P3_05 | QUIET_OUTPUT_MODE | PLANNED | [MICRO_P3_05_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_05_03_QUIET_OUTPUT_MODE.md) |
| P3_05_04 | P3_05 | CLI_REGRESSION_TESTS | PLANNED | [MICRO_P3_05_04](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_05_04_CLI_REGRESSION_TESTS.md) |
| P4_01_01 | P4_01 | ADD_OPTIONAL_METADATA_FIELDS | PLANNED | [MICRO_P4_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_01_ADD_OPTIONAL_METADATA_FIELDS.md) |
| P4_01_02 | P4_01 | BACKWARD_COMPATIBILITY_TESTS | PLANNED | [MICRO_P4_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_02_BACKWARD_COMPATIBILITY_TESTS.md) |
| P4_01_03 | P4_01 | EXPECTED_OUTCOME_MODEL | PLANNED | [MICRO_P4_01_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_03_EXPECTED_OUTCOME_MODEL.md) |
| P4_02_01 | P4_02 | VALIDATE_SCENARIOS_COMMAND | PLANNED | [MICRO_P4_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_02_01_VALIDATE_SCENARIOS_COMMAND.md) |
| P4_02_02 | P4_02 | INVALID_SCENARIO_HANDLING | PLANNED | [MICRO_P4_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_02_02_INVALID_SCENARIO_HANDLING.md) |
| P4_03_01 | P4_03 | CREATE_SCENARIO_TEMPLATE | PLANNED | [MICRO_P4_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_03_01_CREATE_SCENARIO_TEMPLATE.md) |
| P4_03_02 | P4_03 | ADD_ZERO_AND_SINGLE_AXIS_SCENARIOS | PLANNED | [MICRO_P4_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_03_02_ADD_ZERO_AND_SINGLE_AXIS_SCENARIOS.md) |
| P4_03_03 | P4_03 | ADD_STRICT_AND_LENIENT_PROFILE_SCENARIOS | PLANNED | [MICRO_P4_03_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_03_03_ADD_STRICT_AND_LENIENT_PROFILE_SCENARIOS.md) |
| P4_03_04 | P4_03 | AUTHOR_SCENARIO_DOCS | PLANNED | [MICRO_P4_03_04](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_03_04_AUTHOR_SCENARIO_DOCS.md) |
| P5_01_01 | P5_01 | DEFINE_RECEIPT_FILENAME_RULE | PLANNED | [MICRO_P5_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_01_01_DEFINE_RECEIPT_FILENAME_RULE.md) |
| P5_01_02 | P5_01 | APPLY_FILENAME_RULE_TO_RUN_COMMAND | PLANNED | [MICRO_P5_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_01_02_APPLY_FILENAME_RULE_TO_RUN_COMMAND.md) |
| P5_02_01 | P5_02 | PARSE_RECEIPTS_OUT_DIRECTORY | PLANNED | [MICRO_P5_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_01_PARSE_RECEIPTS_OUT_DIRECTORY.md) |
| P5_02_02 | P5_02 | SUMMARIZE_RECEIPT_COUNTS_AND_SCORES | PLANNED | [MICRO_P5_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_02_SUMMARIZE_RECEIPT_COUNTS_AND_SCORES.md) |
| P5_02_03 | P5_02 | SUMMARY_EMPTY_DIR_BEHAVIOR | PLANNED | [MICRO_P5_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_03_SUMMARY_EMPTY_DIR_BEHAVIOR.md) |
| P5_03_01 | P5_03 | GENERATE_SCENARIO_EVIDENCE_MARKDOWN | PLANNED | [MICRO_P5_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_03_01_GENERATE_SCENARIO_EVIDENCE_MARKDOWN.md) |
| P5_03_02 | P5_03 | EVIDENCE_REPORT_TEST | PLANNED | [MICRO_P5_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_03_02_EVIDENCE_REPORT_TEST.md) |
| P5_04_01 | P5_04 | WRITE_RECEIPT_EVIDENCE_DOC | PLANNED | [MICRO_P5_04_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_04_01_WRITE_RECEIPT_EVIDENCE_DOC.md) |
| P6_01_01 | P6_01 | CREATE_BASELINE_STATUS_DOC | PLANNED | [MICRO_P6_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_01_01_CREATE_BASELINE_STATUS_DOC.md) |
| P6_01_02 | P6_01 | CREATE_BASELINE_COMPARISON_TEMPLATE | PLANNED | [MICRO_P6_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_01_02_CREATE_BASELINE_COMPARISON_TEMPLATE.md) |
| P6_01_03 | P6_01 | UPDATE_BENCHMARK_REGRESSION_DOCS | PLANNED | [MICRO_P6_01_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_01_03_UPDATE_BENCHMARK_REGRESSION_DOCS.md) |
| P6_02_01 | P6_02 | WRITE_GOLDEN_UPDATE_POLICY | PLANNED | [MICRO_P6_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_01_WRITE_GOLDEN_UPDATE_POLICY.md) |
| P6_02_02 | P6_02 | IMPLEMENT_GOLDEN_PREVIEW_NO_OVERWRITE | PLANNED | [MICRO_P6_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_02_IMPLEMENT_GOLDEN_PREVIEW_NO_OVERWRITE.md) |
| P6_02_03 | P6_02 | TEST_GOLDEN_FILES_NOT_MUTATED | PLANNED | [MICRO_P6_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_03_TEST_GOLDEN_FILES_NOT_MUTATED.md) |
| P6_03_01 | P6_03 | IMPLEMENT_REGRESSION_COMMAND | PLANNED | [MICRO_P6_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_03_01_IMPLEMENT_REGRESSION_COMMAND.md) |
| P6_03_02 | P6_03 | REGRESSION_COMMAND_TEST | PLANNED | [MICRO_P6_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_03_02_REGRESSION_COMMAND_TEST.md) |
| P7_01_01 | P7_01 | WRITE_QUICKSTART_VALIDATE_AND_RUN | PLANNED | [MICRO_P7_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_01_01_WRITE_QUICKSTART_VALIDATE_AND_RUN.md) |
| P7_01_02 | P7_01 | WRITE_QUICKSTART_RECEIPTS_REPORTS_BENCHMARKS | PLANNED | [MICRO_P7_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_01_02_WRITE_QUICKSTART_RECEIPTS_REPORTS_BENCHMARKS.md) |
| P7_02_01 | P7_02 | REWRITE_README_STRUCTURE | PLANNED | [MICRO_P7_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_02_01_REWRITE_README_STRUCTURE.md) |
| P7_02_02 | P7_02 | VERIFY_README_COMMANDS | PLANNED | [MICRO_P7_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_02_02_VERIFY_README_COMMANDS.md) |
| P7_03_01 | P7_03 | UPDATE_PACKAGE_HANDOFF_MANIFEST | PLANNED | [MICRO_P7_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_01_UPDATE_PACKAGE_HANDOFF_MANIFEST.md) |
| P7_03_02 | P7_03 | SYNC_OPS_CONTEXT_FILES | PLANNED | [MICRO_P7_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_02_SYNC_OPS_CONTEXT_FILES.md) |
| P7_03_03 | P7_03 | VERIFY_PACKAGE_EXCLUDES_GENERATED_FILES | PLANNED | [MICRO_P7_03_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_03_VERIFY_PACKAGE_EXCLUDES_GENERATED_FILES.md) |
| P8_01_01 | P8_01 | RUN_FINAL_STRUCTURE_AUDIT | PLANNED | [MICRO_P8_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_01_RUN_FINAL_STRUCTURE_AUDIT.md) |
| P8_01_02 | P8_01 | RUN_FINAL_VALIDATION_AUDIT | PLANNED | [MICRO_P8_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_02_RUN_FINAL_VALIDATION_AUDIT.md) |
| P8_01_03 | P8_01 | WRITE_P8_FINAL_AUDIT_REPORT | PLANNED | [MICRO_P8_01_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_03_WRITE_P8_FINAL_AUDIT_REPORT.md) |
| P8_02_01 | P8_02 | WRITE_ROADMAP_CLOSEOUT | PLANNED | [MICRO_P8_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_02_01_WRITE_ROADMAP_CLOSEOUT.md) |
| P8_02_02 | P8_02 | DEFINE_FINISHED_WORKBENCH_STATE | PLANNED | [MICRO_P8_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_02_02_DEFINE_FINISHED_WORKBENCH_STATE.md) |
| P8_03_01 | P8_03 | WRITE_ALLOWED_FUTURE_IDEAS | PLANNED | [MICRO_P8_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_03_01_WRITE_ALLOWED_FUTURE_IDEAS.md) |
| P8_03_02 | P8_03 | WRITE_FORBIDDEN_DELAYED_IDEAS | PLANNED | [MICRO_P8_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_03_02_WRITE_FORBIDDEN_DELAYED_IDEAS.md) |

