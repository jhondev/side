# SIDE

## Concepts
- Platform (monorepo)
- Project
- Feature (feat)
- Artifact

## CLI

### commands
```
side new platform <platform_name> (creates a new monorepo with a basic project)
side new project <project_name>

side <project> new feat <feat_name>
side <project> gen <artifact> (proto, db-models)
side <project> db migrate
side <project> db deploy

side setup (install all platform dependencies)
side build (builds the entire platform)
side build <component> (apis, web, mobile)

side init (adds initial config to a existing repo)
```
