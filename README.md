# front_yew_base
rust web前端框架yew基础结构

## PostCss安装tailwindcss步骤
1. 在项目根目录下，创建static目录. cd static
2. npm init -y
3. npm install tailwindcss@latest postcss@latest autoprefixer@latest postcss-cli@latest
4. 新建postcss.config.js,写入如下内容:
```
// postcss.config.js
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  }
}
```

5. npx tailwindcss init
6. 创建一个 CSS 文件，请使用 @tailwind 指令注入 Tailwind 的基础 (base)，组件 (components) 和功能 (utilities) 样式
```
/* ./your-css-folder/styles.css */
@tailwind base;
@tailwind components;
@tailwind utilities;
```
7. 现在打开package.json文件,写入以下脚本
```
"scripts": {
  "build-css": "postcss src/css/tailwind.css -o src/dist/css/styles.css"
}
```
8. 构建最终CSS文件
```
npm run build-css
```

# wasm-pack编译管理

## 工具安装:
1. cargo install wasm-pack
2. cargo install cargo-make
3. rustup target add wasm32-unknown-unknown

## 修改的地方
1. main.rs 文件修改为lib.rs
2. 在Cargo.toml里新增
  ```
  [lib]
  crate-type = ["cdylib", "rlib"]

  #新增依赖wasm-bindgen
  [dependencies]
  wasm-bindgen = "^0.2"
  ```
3. 修改lib.rs
  ```
  use wasm_bindgen::prelude::*;
  #[wasm_bindgen(start)]
  pub fn main(){
      wasm_logger::init(wasm_logger::Config::default());
      yew::start_app::<App>();
  }
  ```
4. index.html新增
  ```
  <script type="module">
    import init from "/pkg/wasm.js";
    init();
  </script>
  ```
5. 在根目录下，新增Makefile.toml
  ```
  [tasks.build]
  args = ["build", "--dev", "--target", "web", "--out-name", "wasm", "--out-dir", "./static/dist/pkg"]
  command = "wasm-pack"
  watch = {ignore_pattern = "static/public/*"}

  # [tasks.serve]
  # command = "simple-http-server"
  # args = ["-i", "./static/public", "-p", "8080", "--nocache", "--try-file", "./static/public/index.html"]
  ```
## 启动服务
1. cargo make build

## 服务器相关
1. nginx单页面配置
  ```
  location / {
      index  index.html index.htm index.php;
      try_files $uri $uri/ /index.html;
  }
  ```