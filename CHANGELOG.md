# Changelog

## [1.1.0] - 2025-08-07

### Added
- Added missing shadcn utils.ts file for demo application
- Enhanced gitignore rules to preserve shadcn utilities

### Fixed
- Fixed MetaSep token mapping bug (was incorrectly mapped to channel token)
- Added missing MetaSep and MetaEnd token registrations in registry
- Improved tokenizer registry functionality for meta formatting tokens

### Changed
- Updated version to 1.1.0 for new release cycle

### Technical Details
- MetaSep token now correctly maps to `<|meta_sep|>` instead of `<|channel|>`
- Registry now properly recognizes MetaSep and MetaEnd formatting tokens
- Demo application now includes required utility functions for UI components
