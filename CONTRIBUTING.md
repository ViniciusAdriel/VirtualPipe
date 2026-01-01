# Project structure

The root directory of this project splits the workflow into three main parts:

- **build/** — scripts and files used to build the application for release  
  - The build process generates the final distributable files inside **dist/**.
- **assets/** — images, media, and other resources used by the code or in the build process.
- **app/** — the actual application  

---

## About the **app/** directory

This directory is organized into:

- **lang/** — language packs  
- **src/** — application logic  
- **structs/** — shared structs used by the code or the UI  
- **ui/** — all Slint UI files

### Contents of the **ui/** directory

**components/**  
Contains the UI components. This corresponds to the main slint file in his folder (in this case, `main.slint`).

**main.slint**  
The main UI entry point, read when the interface is initialized.

**structs.slint** (optional)  
A file that aggregates UI-related structs into a single place to simplify UI development.

Inside **ui/components/**, each component may be represented either as:

- a **single Slint file**, if the component is simple  
- a **folder**, if the component has its own internal structure  
  - In that case, the folder mirrors the structure of the main `ui/` directory (containing a main Slint file and its own child components).  
  - This pattern can be repeated recursively as needed.

---

# Reading the code in order

Once you understand the structure described above, you can start exploring the code from:

```
app/src/main.rs
```

There are comments throughout the codebase that will help guide you as you follow how the project works.
