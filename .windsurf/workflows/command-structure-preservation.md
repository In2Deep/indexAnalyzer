---
description: Command Structure Preservation Workflow
---

# Command Structure Preservation Workflow

This workflow defines strict rules for maintaining the established command structure and CLI interface of the indexAnalyzer application.

## there is no backticks as they break the yaml wrapper

## Core Command Structure

### Base Commands
- remember --name <project> --path <project_dir>: Index all Python files in a project directory
- refresh --name <project> --files <file1.py,file2.py,...>: Refresh memory for specific files in a project
- recall <entity_type> [name] --name <project>: Query for code entities (e.g., functions, classes)
- status --name <project>: Show indexed files and project info
- forget --name <project>: Remove all indexed data for a project

### Vector Commands
- vectorize --name <project> --path <path> --provider <provider> --db <backend> [--batch-size <N>] [--dry-run] [--verbose]: Generate and index code embeddings for a project
- vector-recall --name <project> --query <text> [--top-k <N>] [--model <provider>] [--db <backend>] [--json]: Semantic similarity search over indexed code entities

## Rules for Command Structure

1. **Preserve Existing Command Names and Structure**
   - All command names must remain exactly as specified above
   - Command hierarchies and subcommand relationships must not change
   - New commands may only be added, not renamed or restructured

2. **Preserve Flag Names and Behavior**
   - All flag names must remain exactly as documented
   - Required vs. optional status of flags must not change
   - Flag behavior and semantics must remain consistent

3. **Preserve Parameter Naming**
   - Parameter names in code must match CLI flag names
   - Parameter types must remain consistent

4. **Implementation Requirements**
   - All implementations must honor the documented behavior
   - All commands must support the flags as documented
   - Default values must align with documentation

## Implementation Process

When implementing or modifying commands:

1. First review this workflow and the README.md documentation
2. Ensure all tests validate the exact command structure specified here
3. Implement functionality that adheres strictly to this interface
4. Verify implementation matches documentation before committing

## Prohibited Actions

- Changing command names or hierarchies
- Renaming flags or parameters
- Changing required/optional status of parameters
- Altering the behavior of existing commands
- Adding undocumented flags or parameters

This workflow is absolute. The command structure is finalized and must be preserved exactly as documented.