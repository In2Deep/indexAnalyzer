---
description: Documentation Preservation and Extension Workflow
---

# Documentation Preservation and Extension Workflow

This workflow defines strict rules for how documentation files should be handled, particularly README.md files and other critical documentation.

## Core Rules

1. **NEVER DELETE EXISTING DOCUMENTATION CONTENT**
   - Documentation may only be extended, not reduced or rewritten
   - All existing sections, commands, examples, and structure must be preserved exactly as they are
   - Formatting, headings, and organization must remain intact

2. **Acceptable Changes**
   - Adding new sections at appropriate locations
   - Extending existing sections with new information
   - Fixing typos or broken links
   - Adding clarifications that don't change the original meaning

3. **Required Process**
   - Before modifying any documentation file, create a backup
   - Show a diff of proposed changes before committing
   - Explicitly call out any sections being modified and justify the changes
   - Get explicit approval for documentation changes

4. **Prohibited Actions**
   - Rewriting existing content
   - Removing sections, examples, or command descriptions
   - Changing command syntax or examples
   - Restructuring the document organization
   - "Improving" or "cleaning up" existing content without explicit permission

## Implementation

When asked to update documentation:

1. First show the current state of the documentation
2. Propose specific additions (not changes) to make
3. Wait for explicit approval before making any changes
4. After changes, show a diff to confirm only additions were made

Remember: Documentation represents significant effort and design decisions. Respect the existing structure and content as immutable unless explicitly directed otherwise.
