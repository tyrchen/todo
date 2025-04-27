# Memory Bank Index

## Project Overview
- [Project Brief](projectbrief.md) - Core project goals and requirements
- [Tech Context](techContext.md) - Technical context and stack information
- [Product Context](productContext.md) - Product vision and requirements
- [System Patterns](systemPatterns.md) - Architectural patterns used in the project

## Current Status
- [Active Context](activeContext.md) - Current project focus and status
- [Tasks](tasks.md) - Active and completed tasks

## Refactoring Plan
- [Refactoring Plan](refactoring_plan.md) - Detailed plan for clean code refactoring
- [Code Examples](code_examples.md) - Specific code examples that need refactoring
- [Constants Template](constants_template.md) - Template for the new constants.rs file

## Progress
- [Progress](progress.md) - Development progress log

## How to Use This Memory Bank

1. Start with the **Project Overview** files to understand the overall goals
2. Check the **Active Context** for the current project focus
3. Review the **Tasks** file to understand what needs to be done
4. For the current refactoring initiative, refer to:
   - The detailed **Refactoring Plan**
   - Specific **Code Examples** that need improvement
   - The **Constants Template** for implementing the new constants module
5. Track progress in the **Progress** file

## Refactoring Guidelines

1. **Follow Clean Code Principles**
   - Single Responsibility Principle
   - DRY (Don't Repeat Yourself)
   - Meaningful names
   - Replace magic numbers with constants

2. **Phased Approach**
   - Start with the constants file
   - Then address core model improvements
   - Next, improve component architecture
   - Finally, enhance tests and documentation

3. **Testing Strategy**
   - Unit tests for core functionality
   - Run tests after each refactoring step
   - Add assertions to verify behavior

4. **Documentation Strategy**
   - Document public API thoroughly
   - Add examples for complex functions
   - Document architectural decisions
