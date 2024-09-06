# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.14'>v0.1.14</a>
### 2024-09-06

### Chores
+ Rust 1.81 linting, [466fd826](https://github.com/mrjackwills/havn/commit/466fd8263cd0b4c2fc2c41d71e706b0984e1f4ce)
+ dependencies updated, [8d5490c8](https://github.com/mrjackwills/havn/commit/8d5490c8220244326dc653a807615dcc562eab00)

### Docs
+ README.md aarch64 cross build typo, [69df36ba](https://github.com/mrjackwills/havn/commit/69df36bad2a284d989c7d232ee6558121931fe2a)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.13'>v0.1.13</a>
### 2024-07-25

### Chores
+ .devcontainer updated, [d79cbe37](https://github.com/mrjackwills/havn/commit/d79cbe37053dc1f41ca843e64e75612a369e9978)
+ dependencies updated, [1809faa9](https://github.com/mrjackwills/havn/commit/1809faa95e28daf61330b72333fd97a03a721002)

### Fixes
+ install.sh use curl, [ba6218c7](https://github.com/mrjackwills/havn/commit/ba6218c7b365bfbf1ce025088e8e26caac017290)
+ dockerfile lowercase stages, [98e5f722](https://github.com/mrjackwills/havn/commit/98e5f72227038d8b5927d458fe524c6d27e21fc2)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.12'>v0.1.12</a>
### 2024-06-17

### Chores
+ dependencies updated, [eb48e49f](https://github.com/mrjackwills/havn/commit/eb48e49f4efe059fc163b3867d5410cd191f90b6)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.11'>v0.1.11</a>
### 2024-05-06

### Chores
+ dependencies updated, [7c396072](https://github.com/mrjackwills/havn/commit/7c39607227690f91090d2beeb6eee62b53fbb43d), [81f1b9e6](https://github.com/mrjackwills/havn/commit/81f1b9e6fb5998e02a5ebc8904bf5bb5d2ca5ffb), [af2c9d22](https://github.com/mrjackwills/havn/commit/af2c9d226925b2571620d0306b72e80764a96ea4)
+ Rust 1.78.0 linting, [ed6b20f6](https://github.com/mrjackwills/havn/commit/ed6b20f6a0284dd7c0cc858acff14bda5a15a31a)

### Fixes
+ Fix `arch` command not found, thanks [Chleba](https://github.com/Chleba), [b4d4c2b5](https://github.com/mrjackwills/havn/commit/b4d4c2b51d8c492be97d7aa02d84e323acdc8a69)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.10'>v0.1.10</a>
### 2024-04-15

### Chores
+ Dependencies updated, [bec41d4b](https://github.com/mrjackwills/havn/commit/bec41d4b8f176b0f99ec74dcba57aef80f458d9a), [86254c49](https://github.com/mrjackwills/havn/commit/86254c4924e78b621cfe5b2d716d692daa455dd0)

### Docs
+ add arch linux instructions, thanks [orhun](https://github.com/orhun), [c5d49803](https://github.com/mrjackwills/havn/commit/c5d49803deb415189c38ce9fe29e93501a4f5cf6)

### Fixes
+ `get_extra_ips()` errant comma addition removed, [dcb05f1c](https://github.com/mrjackwills/havn/commit/dcb05f1c623ba26c151c6f2b79a02b9559f9b2cb)

### Refactors
+ Refactor main fn to use let-else, thanks [Thorsten Hans](https://github.com/ThorstenHans), [4f0b5b3b](https://github.com/mrjackwills/havn/commit/4f0b5b3b4ff199cb84566c9cb082a81bca797359)
+ only match in the fmt::Display impl's when not in MONOCHROME mode, [11a5abe5](https://github.com/mrjackwills/havn/commit/11a5abe5b9e3650918d5dac927d37f697c5443e7)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.9'>v0.1.9</a>
### 2024-03-31

### Chores
+ dependencies updated, [36a918b1](https://github.com/mrjackwills/havn/commit/36a918b150098da934348f3526aee754d4d7ad17), [bdd368f9](https://github.com/mrjackwills/havn/commit/bdd368f9dd62df8a98bff78d753749bae3a54c2e)
+ GitHub workflow dependency updated, [10978f92](https://github.com/mrjackwills/havn/commit/10978f926352348b6f110ac64df9b2713536315a)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.8'>v0.1.8</a>
### 2024-02-12

### Chores
+ .devcontainer updated, [04f23e5e](https://github.com/mrjackwills/havn/commit/04f23e5e3d87334ccf210097e7c0f6abf0d635eb)
+ create_release v0.5.4, [6cbd5114](https://github.com/mrjackwills/havn/commit/6cbd51145a4a108ed98e178a573c990ee1bae118), [fae25ff6](https://github.com/mrjackwills/havn/commit/fae25ff6520c1125dfcec0e5c32f6fa63f438b50)
+ GitHub workflow action dependency bump, [ee2eeb6b](https://github.com/mrjackwills/havn/commit/ee2eeb6bdc7f8187e1522f4dcf5f62830fab0c9b)
+ dependencies updated, [644ed7e9](https://github.com/mrjackwills/havn/commit/644ed7e9632d408cd96f1e7b7f085d46b57aa836), [6242b3ce](https://github.com/mrjackwills/havn/commit/6242b3ce622d1de340341f3dcd2acb586370d93a), [fb2db506](https://github.com/mrjackwills/havn/commit/fb2db50695f38656c37f1b816c0c5db695c0d236)
+ files formatted, [6b6b8335](https://github.com/mrjackwills/havn/commit/6b6b83351bb8dbacf8fce1adeb0404ae743d9852)

### Fixes
+ .gitattributes, [a445136e](https://github.com/mrjackwills/havn/commit/a445136e086b42b7973ff7951b78d9826741e995)

### Refactors
+ text_color map removed, [d1b36d31](https://github.com/mrjackwills/havn/commit/d1b36d31f09407776efcf4dce415a3ab293e8e8f)
+ GitHub action workflow improved, [1c4f7a2c](https://github.com/mrjackwills/havn/commit/1c4f7a2c31b20e61e3109abc975116e5b05b5a74)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.7'>v0.1.7</a>
### 2023-12-30

### Chores
+ .devcontainer updated, [eb887f4b](https://github.com/mrjackwills/havn/commit/eb887f4b0344919c24be2136f874f3eb5bebeaa6)
+ dependencies updated, [4b049b59](https://github.com/mrjackwills/havn/commit/4b049b59f1fab35ffdd6c360ce4a6337bf5f55ae)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.6'>v0.1.6</a>
### 2023-11-17

### Chores
+ lints moved to Cargo.toml, [4e397e10](https://github.com/mrjackwills/havn/commit/4e397e107355f41631b893a367e6a9cc5c9a87c4)
+ .devcontainer updated, [f2ca3bc1](https://github.com/mrjackwills/havn/commit/f2ca3bc12a4bed2ea43af3ecf052a4cdb5478427)
+ dependencies updated, [b10346bf](https://github.com/mrjackwills/havn/commit/b10346bfb13452ec104985d632963c0df9bab818), [7a841f81](https://github.com/mrjackwills/havn/commit/7a841f81dca151a1a23b0d8fe0c37878b5664c19)
+ devcontainer plugin updated, [21e58543](https://github.com/mrjackwills/havn/commit/21e58543767b9d15de3b8bdcad04ff88466e8d0f)

### Refactors
+ GitHub workflow use matrix concurrency, [2767aae0](https://github.com/mrjackwills/havn/commit/2767aae04df7dc09fa8b09d7da209759761b5fb3)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.5'>v0.1.5</a>
### 2023-10-09

### Chores
+ dependencies updated, [d61cf8d2](https://github.com/mrjackwills/havn/commit/d61cf8d2e8056b13b6de9d8a664495906296cfac), [077fd74d](https://github.com/mrjackwills/havn/commit/077fd74db2cb9609ae298c0178c2d5985e4945ee), [2f3829fc](https://github.com/mrjackwills/havn/commit/2f3829fc7fca5a9f31db56241d9ef0913c121297) 

### Refactors
+ Spinner fn new, use an atomic swap for spinner.start(), [9a0faa12](https://github.com/mrjackwills/havn/commit/9a0faa129f34da32823178689448a1a9eebd042a)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.4'>v0.1.4</a>
### 2023-08-25

### Chores
+ dependencies updated, [ac14f79c](https://github.com/mrjackwills/havn/commit/ac14f79ccedd6f35967ecc41b5ea7ce3e9066d17), [c3f526d3](https://github.com/mrjackwills/havn/commit/c3f526d37d95534ae736a76aa279193bca6c1707)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.3'>v0.1.3</a>
### 2023-07-30

### Chores
+ dependencies updated, [67c64d28](https://github.com/mrjackwills/havn/commit/67c64d28ab728614b85fa8a3f93a270e226da4e2)
+ create_release v0.3.0, [594a901e](https://github.com/mrjackwills/havn/commit/594a901e374a155736f6f755aa5d71b9ecfbc5fe)

### Docs
+ readme updated, [43f23a47](https://github.com/mrjackwills/havn/commit/43f23a4780d102a2af20d3c1e0846509389484f5)

### Features
+ monochrome mode, closes [#1](https://github.com/mrjackwills/havn/issues/1), [1efd3905](https://github.com/mrjackwills/havn/commit/1efd39050fa4471d773d5a84607514952e9079d9)

# <a href='https://github.com/mrjackwills/havn/releases/tag/v0.1.2'>v0.1.2</a>
### 2023-06-04

### Chores
+ dependencies updated, [26d4197f](https://github.com/mrjackwills/havn/commit/26d4197f4cf64e05a4973f57e53d603d4da72535), [8a3ea4e8](https://github.com/mrjackwills/havn/commit/8a3ea4e86c99e3d282723ea339335f2f81936d1f), [2e693022](https://github.com/mrjackwills/havn/commit/2e69302289315cf87adeb142c3737da7e347c897)

### Docs
+ readme updated, [b2ae7a40](https://github.com/mrjackwills/havn/commit/b2ae7a40a7796e060ac7d4d7838bc60a445d50ff)
+ cli arguments description updated, [bed82a62](https://github.com/mrjackwills/havn/commit/bed82a62e45b913001dbf358f463f01f79c7503e)

### Refactors
+ spinner, frames into a const, [43328651](https://github.com/mrjackwills/havn/commit/4332865178fa8d78d5f2c0b95adb5e40622301ea)

### Reverts
+ .devcontainer sparse protocol env removed, behaviour is now default in Rust 1.70, [2ecf56a4](https://github.com/mrjackwills/havn/commit/2ecf56a4f233423230e8f661068e7429634e0165)

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
