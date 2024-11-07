# Dynaplate

Dynaplate is a powerful templating tool that transforms simple configuration files into dynamic templates, allowing you
to create versatile project setups regardless of programming language or framework. With custom commands and flexible
configurations, Dynaplate is designed to streamline project initialization.

## Configuration Options

Dynaplate provides a range of configuration options to tailor your templates:

- **Pre-commands**: Commands that run before the template processing begins.
- **Environment Variables**: Define environment variables to be included in commands.
- **Variables**: Set interactive values that can be referenced in commands and files using {{variable_name}}.
- **Dependencies**: Specify project dependencies along with installation commands.
- **Template Files**: Copy predefined files or folders into the working directory.
- **Post-commands**: Commands that run after all template processing is complete.

If you don't provide a working directory as an argument when running Dynaplate, it will prompt you for one.

This flexible configuration approach allows you to dynamically generate templates. With support for custom commands, you
can create templates that fit any workflow. For instance, you might set up Jinja template files and include a
post-command to populate them with specific values.

## Default Variables

Dynaplate includes a default variable:

- `working_dir`: The directory where the project will be initialized, and all commands will be executed.
- ... more coming soon

## Possible variable types

Several variable types are available:

- string (any text)
- boolean (`true` or `false`)
- select (specify `options` with array of strings)

## Possible condition operators

Several condition operators are available (more will be added: see roadmap below):

- equals
- notequals

## Example Configuration

Here’s an example configuration for a Node.js and Express backend project, featuring essential environment variables,
dependencies, and file structure:

```yaml
name: "Node setup"
description: "Initial setup for a Node.js and Express backend project with essential environment variables, dependencies, and file structure."
version: "1.0.0"

pre_commands:
  - name: "Use Node 22"
    command: "nvm use 22"

environment:
  - name: "ENVIRONMENT"
    value: "LOCAL"

variables:
  - name: "use_docker"
    description: "Do you want to add Docker support?"
    default: false
    type: boolean
  - name: "database"
    description: "Choose the database to configure"
    default: "PostgreSQL"
    type: select
    options:
      - PostgreSQL
      - MySQL
      - SQLite

dependencies:
  - name: "express"
    command: "npm install express"
  - name: "dotenv"
    command: "npm install dotenv"
  - name: "database_client_pg"
    command: npm install pg
    conditions:
      - variable: "database"
        operator: "equals"
        value: "PostgreSQL"
  - name: "database_client_mysql"
    command: npm install mysql2
    conditions:
      - variable: "database"
        operator: "equals"
        value: "MySQL"
  - name: "database_client_sqlite"
    command: npm install sqlite3
    conditions:
      - variable: "database"
        operator: "equals"
        value: "SQLite"
  - name: "nodemon"
    command: "npm install --save-dev nodemon"
    conditions:
      - variable: "use_docker"
        operator: "equals"
        value: false

template_files:
  - name: "node_project_files"
    file_type: "folder"
    source: "/home/user/Templates/node_project"
    destination: "{{working_dir}}"
  - name: "Dockerfile"
    file_type: "file"
    source: "/home/user/Templates/Docker/node/Dockerfile"
    destination: "{{working_dir}}/Dockerfile"

post_commands:
  - name: "initialize_git"
    command: "git init"
  - name: "create_env_file"
    command: "cp {{working_dir}}/.env_example {{working_dir}}/.env"
  - name: "run_server"
    command: "npm run dev"
```

Note: if you need to set a variable programmatically, set the key `value` to what you want, depending on the type:
- `string`: `any text here`
- `boolean`: `true` or `false`
- `select`: `existing option here`

## Roadmap (Non-Exhaustive)

Dynaplate is continually evolving, and future enhancements may include:

- [ ] Support for template files from URLs, Git repositories, etc.
- [ ] More efficient dependency processing by merging similar install commands.
- [ ] Expanded support for conditional operators (e.g., not equals, greater than, less than).
- [ ] Clear, comprehensive documentation.
- [ ] Example configurations to demonstrate use cases.
- [ ] Improved error handling (all validation at once)
- [ ] Improved output (colors, structured, etc.)

If you have any suggestions, want to contribute or give feedback: open an issue or contact me (see my profile)