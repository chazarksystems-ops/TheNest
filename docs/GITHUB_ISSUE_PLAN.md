# GitHub Issue & Milestone Plan

This plan organizes the remaining implementation tasks into milestones and issues.

## Milestones

1. **Milestone P3 — CLI Command Surface**
   - Purpose: Evolve demo CLI to hive_workbench CLI with robust manual parsing.
2. **Milestone P4 — Scenario Authoring**
   - Purpose: Enhance metadata, expected outcomes, schema compatibility, templates.
3. **Milestone P5 — Receipt Evidence**
   - Purpose: Predictable filenames, summarization commands, scenario reports.
4. **Milestone P6 — Regression Baselines**
   - Purpose: Status files, golden update policies, golden previews.
5. **Milestone P7 — Packaging and Docs**
   - Purpose: Normalising README, QUICKSTART, and packaging scripts.
6. **Milestone P8 — Final Audit and Closeout**
   - Purpose: Audits, closeout checklists, parking lots.

## Suggest Labels

- phase:P3, phase:P4, phase:P5, phase:P6, phase:P7, phase:P8
- workstream:cli, workstream:scenarios, workstream:receipts, workstream:regression, workstream:docs, workstream:ops
- gent-ready, microtask, locked, 
o-runtime

## Issue List by Milestone
### Issue: TASK_P3_01_CONTEXT_AND_CLI_SPEC
- **Phase:** P3
- **Workstream:** 07_ops_agent_dispatch
- **Linked Task:** [TASK_P3_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_01_CONTEXT_AND_CLI_SPEC.md)
- **Microtasks Included:**
  - MICRO_P3_01_01_REMOVE_RUNTIME_QUEUE_REFERENCES
  - MICRO_P3_01_02_CREATE_CLI_ARG_SPEC
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P3, workstream:ops, gent-ready
### Issue: TASK_P3_02_WORKBENCH_BINARY
- **Phase:** P3
- **Workstream:** 02_workbench_cli
- **Linked Task:** [TASK_P3_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_02_WORKBENCH_BINARY.md)
- **Microtasks Included:**
  - MICRO_P3_02_01_DECIDE_DEMO_VS_HIVE_WORKBENCH
  - MICRO_P3_02_02_UPDATE_CARGO_BINARY_TARGET
  - MICRO_P3_02_03_UPDATE_VALIDATE_SCRIPT_BINARY_REFERENCES
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P3, workstream:workbench, gent-ready
### Issue: TASK_P3_03_MANUAL_CLI_PARSER
- **Phase:** P3
- **Workstream:** 02_workbench_cli
- **Linked Task:** [TASK_P3_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_03_MANUAL_CLI_PARSER.md)
- **Microtasks Included:**
  - MICRO_P3_03_01_PARSE_COMMAND_NAME
  - MICRO_P3_03_02_PARSE_OUTPUT_FLAG
  - MICRO_P3_03_03_STABLE_USAGE_ERRORS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P3, workstream:workbench, gent-ready
### Issue: TASK_P3_04_COMMANDS_LIST_RUN_RUNFILE_SUITE
- **Phase:** P3
- **Workstream:** 02_workbench_cli
- **Linked Task:** [TASK_P3_04](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_04_COMMANDS_LIST_RUN_RUNFILE_SUITE.md)
- **Microtasks Included:**
  - MICRO_P3_04_01_LIST_SCENARIOS
  - MICRO_P3_04_02_RUN_NAMED_SCENARIO
  - MICRO_P3_04_03_RUN_FILE_SCENARIO
  - MICRO_P3_04_04_RUN_SCENARIO_SUITE
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P3, workstream:workbench, gent-ready
### Issue: TASK_P3_05_OUTPUT_MODES_AND_TESTS
- **Phase:** P3
- **Workstream:** 02_workbench_cli
- **Linked Task:** [TASK_P3_05](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_05_OUTPUT_MODES_AND_TESTS.md)
- **Microtasks Included:**
  - MICRO_P3_05_01_HUMAN_OUTPUT_MODE
  - MICRO_P3_05_02_JSON_OUTPUT_MODE
  - MICRO_P3_05_03_QUIET_OUTPUT_MODE
  - MICRO_P3_05_04_CLI_REGRESSION_TESTS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P3, workstream:workbench, gent-ready
### Issue: TASK_P4_01_METADATA_SCHEMA
- **Phase:** P4
- **Workstream:** 03_scenario_library
- **Linked Task:** [TASK_P4_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P4/TASK_P4_01_METADATA_SCHEMA.md)
- **Microtasks Included:**
  - MICRO_P4_01_01_ADD_OPTIONAL_METADATA_FIELDS
  - MICRO_P4_01_02_BACKWARD_COMPATIBILITY_TESTS
  - MICRO_P4_01_03_EXPECTED_OUTCOME_MODEL
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P4, workstream:scenario, gent-ready
### Issue: TASK_P4_02_VALIDATE_SCENARIOS_COMMAND
- **Phase:** P4
- **Workstream:** 03_scenario_library
- **Linked Task:** [TASK_P4_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P4/TASK_P4_02_VALIDATE_SCENARIOS_COMMAND.md)
- **Microtasks Included:**
  - MICRO_P4_02_01_VALIDATE_SCENARIOS_COMMAND
  - MICRO_P4_02_02_INVALID_SCENARIO_HANDLING
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P4, workstream:scenario, gent-ready
### Issue: TASK_P4_03_TEMPLATE_AND_LIBRARY_EXPANSION
- **Phase:** P4
- **Workstream:** 03_scenario_library
- **Linked Task:** [TASK_P4_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P4/TASK_P4_03_TEMPLATE_AND_LIBRARY_EXPANSION.md)
- **Microtasks Included:**
  - MICRO_P4_03_01_CREATE_SCENARIO_TEMPLATE
  - MICRO_P4_03_02_ADD_ZERO_AND_SINGLE_AXIS_SCENARIOS
  - MICRO_P4_03_03_ADD_STRICT_AND_LENIENT_PROFILE_SCENARIOS
  - MICRO_P4_03_04_AUTHOR_SCENARIO_DOCS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P4, workstream:scenario, gent-ready
### Issue: TASK_P5_01_PREDICTABLE_RECEIPT_FILENAMES
- **Phase:** P5
- **Workstream:** 04_receipts_evidence
- **Linked Task:** [TASK_P5_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P5/TASK_P5_01_PREDICTABLE_RECEIPT_FILENAMES.md)
- **Microtasks Included:**
  - MICRO_P5_01_01_DEFINE_RECEIPT_FILENAME_RULE
  - MICRO_P5_01_02_APPLY_FILENAME_RULE_TO_RUN_COMMAND
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P5, workstream:receipts, gent-ready
### Issue: TASK_P5_02_RECEIPT_SUMMARY_COMMAND
- **Phase:** P5
- **Workstream:** 04_receipts_evidence
- **Linked Task:** [TASK_P5_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P5/TASK_P5_02_RECEIPT_SUMMARY_COMMAND.md)
- **Microtasks Included:**
  - MICRO_P5_02_01_PARSE_RECEIPTS_OUT_DIRECTORY
  - MICRO_P5_02_02_SUMMARIZE_RECEIPT_COUNTS_AND_SCORES
  - MICRO_P5_02_03_SUMMARY_EMPTY_DIR_BEHAVIOR
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P5, workstream:receipts, gent-ready
### Issue: TASK_P5_03_SCENARIO_EVIDENCE_REPORT
- **Phase:** P5
- **Workstream:** 04_receipts_evidence
- **Linked Task:** [TASK_P5_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P5/TASK_P5_03_SCENARIO_EVIDENCE_REPORT.md)
- **Microtasks Included:**
  - MICRO_P5_03_01_GENERATE_SCENARIO_EVIDENCE_MARKDOWN
  - MICRO_P5_03_02_EVIDENCE_REPORT_TEST
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P5, workstream:receipts, gent-ready
### Issue: TASK_P5_04_RECEIPT_EVIDENCE_DOCS
- **Phase:** P5
- **Workstream:** 04_receipts_evidence
- **Linked Task:** [TASK_P5_04](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P5/TASK_P5_04_RECEIPT_EVIDENCE_DOCS.md)
- **Microtasks Included:**
  - MICRO_P5_04_01_WRITE_RECEIPT_EVIDENCE_DOC
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P5, workstream:receipts, gent-ready
### Issue: TASK_P6_01_BASELINE_STATUS_AND_COMPARISON
- **Phase:** P6
- **Workstream:** 05_regression_baselines
- **Linked Task:** [TASK_P6_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P6/TASK_P6_01_BASELINE_STATUS_AND_COMPARISON.md)
- **Microtasks Included:**
  - MICRO_P6_01_01_CREATE_BASELINE_STATUS_DOC
  - MICRO_P6_01_02_CREATE_BASELINE_COMPARISON_TEMPLATE
  - MICRO_P6_01_03_UPDATE_BENCHMARK_REGRESSION_DOCS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P6, workstream:regression, gent-ready
### Issue: TASK_P6_02_GOLDEN_PREVIEW_POLICY_AND_COMMAND
- **Phase:** P6
- **Workstream:** 05_regression_baselines
- **Linked Task:** [TASK_P6_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P6/TASK_P6_02_GOLDEN_PREVIEW_POLICY_AND_COMMAND.md)
- **Microtasks Included:**
  - MICRO_P6_02_01_WRITE_GOLDEN_UPDATE_POLICY
  - MICRO_P6_02_02_IMPLEMENT_GOLDEN_PREVIEW_NO_OVERWRITE
  - MICRO_P6_02_03_TEST_GOLDEN_FILES_NOT_MUTATED
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P6, workstream:regression, gent-ready
### Issue: TASK_P6_03_REGRESSION_COMMAND
- **Phase:** P6
- **Workstream:** 05_regression_baselines
- **Linked Task:** [TASK_P6_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P6/TASK_P6_03_REGRESSION_COMMAND.md)
- **Microtasks Included:**
  - MICRO_P6_03_01_IMPLEMENT_REGRESSION_COMMAND
  - MICRO_P6_03_02_REGRESSION_COMMAND_TEST
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P6, workstream:regression, gent-ready
### Issue: TASK_P7_01_QUICKSTART_GUIDE
- **Phase:** P7
- **Workstream:** 06_docs_packaging
- **Linked Task:** [TASK_P7_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P7/TASK_P7_01_QUICKSTART_GUIDE.md)
- **Microtasks Included:**
  - MICRO_P7_01_01_WRITE_QUICKSTART_VALIDATE_AND_RUN
  - MICRO_P7_01_02_WRITE_QUICKSTART_RECEIPTS_REPORTS_BENCHMARKS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P7, workstream:docs, gent-ready
### Issue: TASK_P7_02_README_NORMALIZATION
- **Phase:** P7
- **Workstream:** 06_docs_packaging
- **Linked Task:** [TASK_P7_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P7/TASK_P7_02_README_NORMALIZATION.md)
- **Microtasks Included:**
  - MICRO_P7_02_01_REWRITE_README_STRUCTURE
  - MICRO_P7_02_02_VERIFY_README_COMMANDS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P7, workstream:docs, gent-ready
### Issue: TASK_P7_03_PACKAGING_MANIFEST_AND_CONTEXT_SYNC
- **Phase:** P7
- **Workstream:** 06_docs_packaging
- **Linked Task:** [TASK_P7_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P7/TASK_P7_03_PACKAGING_MANIFEST_AND_CONTEXT_SYNC.md)
- **Microtasks Included:**
  - MICRO_P7_03_01_UPDATE_PACKAGE_HANDOFF_MANIFEST
  - MICRO_P7_03_02_SYNC_OPS_CONTEXT_FILES
  - MICRO_P7_03_03_VERIFY_PACKAGE_EXCLUDES_GENERATED_FILES
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P7, workstream:docs, gent-ready
### Issue: TASK_P8_01_FINAL_AUDIT
- **Phase:** P8
- **Workstream:** 08_final_audit_closeout
- **Linked Task:** [TASK_P8_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P8/TASK_P8_01_FINAL_AUDIT.md)
- **Microtasks Included:**
  - MICRO_P8_01_01_RUN_FINAL_STRUCTURE_AUDIT
  - MICRO_P8_01_02_RUN_FINAL_VALIDATION_AUDIT
  - MICRO_P8_01_03_WRITE_P8_FINAL_AUDIT_REPORT
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P8, workstream:final, gent-ready
### Issue: TASK_P8_02_ROADMAP_CLOSEOUT
- **Phase:** P8
- **Workstream:** 08_final_audit_closeout
- **Linked Task:** [TASK_P8_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P8/TASK_P8_02_ROADMAP_CLOSEOUT.md)
- **Microtasks Included:**
  - MICRO_P8_02_01_WRITE_ROADMAP_CLOSEOUT
  - MICRO_P8_02_02_DEFINE_FINISHED_WORKBENCH_STATE
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P8, workstream:final, gent-ready
### Issue: TASK_P8_03_FUTURE_PARKING_LOT
- **Phase:** P8
- **Workstream:** 08_final_audit_closeout
- **Linked Task:** [TASK_P8_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P8/TASK_P8_03_FUTURE_PARKING_LOT.md)
- **Microtasks Included:**
  - MICRO_P8_03_01_WRITE_ALLOWED_FUTURE_IDEAS
  - MICRO_P8_03_02_WRITE_FORBIDDEN_DELAYED_IDEAS
- **Acceptance Criteria:** Validation script compiles and passes. All microtask conditions satisfied.
- **Labels:** phase:P8, workstream:final, gent-ready

