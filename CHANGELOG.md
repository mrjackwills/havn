### Chores
+ dependencies updated, [26d4197f4cf64e05a4973f57e53d603d4da72535], [8a3ea4e86c99e3d282723ea339335f2f81936d1f], [2e69302289315cf87adeb142c3737da7e347c897]

### Docs
+ readme updated, [b2ae7a40a7796e060ac7d4d7838bc60a445d50ff]
+ cli arguments description updated, [bed82a62e45b913001dbf358f463f01f79c7503e]

### Refactors
+ spinner, frames into a const, [4332865178fa8d78d5f2c0b95adb5e40622301ea]

### Reverts
+ .devcontainer sparse protocol env removed, behaviour is now default in Rust 1.70, [2ecf56a4f233423230e8f661068e7429634e0165]

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.1'>v0.1.1</a>
### 2023-05-21

### Chores
+ dependencies updated, [a17b140c](https://github.com/mrjackwills/havn/commit/a17b140c4cab583b9471f271a429a98f981fba2c)

### Docs
+ README.md tweaked, [094b5ff5](https://github.com/mrjackwills/havn/commit/094b5ff5b3698ba8210bfcb47e11eb70e3613937), [bad851fd](https://github.com/mrjackwills/havn/commit/bad851fd32f21a18c46dfc88ec00698f9ee61607)
+ readme webp added, [df35213d](https://github.com/mrjackwills/havn/commit/df35213d5f44ed14e79e3ad3ecbd54a3e94d3740)
+ comment improved, [bd207421](https://github.com/mrjackwills/havn/commit/bd20742199411511f98383cc63d77e6ac742183b)

### Fixes
+ cli_arg descriptions improved, [78e0e865](https://github.com/mrjackwills/havn/commit/78e0e86522ae51313e336a5ff9b6763d1a08c48d)

### Refactors
+ spinner use [char;10] instead of enum, [8d38f25d](https://github.com/mrjackwills/havn/commit/8d38f25d7a7fdd4df08bc5ec6f817bbf61da421b)
+ color enum just store digit, add escape chars in `write!()`, [517f422f](https://github.com/mrjackwills/havn/commit/517f422fc5252719b4006e3ef44852f5de666c3a)

### Tests
+ refactor parse_arg tests, [cd6c27b9](https://github.com/mrjackwills/havn/commit/cd6c27b9cb0a077d33afc866a6c2d2295070027b)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.0'>v0.1.0</a>
### 2023-05-18

### Features
+ init commit