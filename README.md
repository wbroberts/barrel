# Barrel

A simple CLI tool that creates barrel files for TS directories. This is a project to help me learn working with the filesystem in Rust.

## How To Use

### Example Repo

```txt
src
  - components
    - Header.tsx
    - Header.test.tsx
    - Header.stories.tsx
  - server.ts
  - utils
    - index.ts
    - <...files.ts>
```

#### No Args From Top Level

```bash
barrel src # Adds an index.ts file in the src directory
```

Outputs:

```typescript
export * from 'server';
export * from 'utils';

```

The `components` directory was avoided because there is not a barrel file in it.

To create a barrel file for `components`:

```bash
barrel src/components # Adds an index.ts file in src/components
```

Outputs:

```typescript
// Named export example:
export * from 'Header';

// Default export example:
export { default as Header } from 'Header';
```
