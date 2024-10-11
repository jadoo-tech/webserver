This is an axum project with askama for templating, tailwindcss for css.

To use, first ensure rust is installed so you can use cargo (rust-lang.org/tools/install). 
Then, ensure tailwindcss is installed, either from their manual standalone installer (https://tailwindcss.com/docs/installation) or through npm i -g tailwindcss.

Then, set up the .env file with the correct variables. Here's mine as reference:
```
ENV=dev
tailwind_path="C:\\Users\\ronit\\AppData\\Roaming\\npm\\tailwind.cmd"
```
You can use `which tailwind` to find the path to the tailwind executable on your system. Ensure to use the correct backslash or forward slash in the path. (The above is windows, use a single forward slash on linux/mac)

Then, you can use `cargo run` to start the webserver, and navigate to the various pages available (/ for home, /project/xxx for specific projects. See the routes.rs file)

If you'd like hot reloading, I'd recommend installing `cargo watch` (`cargo install cargo-watch`) and running `cargo watch --ignore static/styles.css -x run` so you don't have to restart the server every time you make changes. 
