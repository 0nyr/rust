# Jupyter for rust

## Install a new Jupyter kernel for rust

For running Rust code in a more interactive way, you can use the `evcxr` Jupyter kernel. Here's how to install and use it:

1. Install the `evcxr_jupyter` kernel:

<pre><div class="bg-black rounded-md mb-4"><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-sh">cargo install evcxr_jupyter
</code></div></div></pre>

```shell
(base) [onyr@kenzael jupyter]$ cargo install evcxr_jupyter
    Updating crates.io index
  Downloaded evcxr_jupyter v0.14.2
  Downloaded 1 crate (53.7 KB) in 0.55s
  Installing evcxr_jupyter v0.14.2
[...]
Compiling evcxr_jupyter v0.14.2
    Finished release [optimized] target(s) in 1m 25s
  Installing /home/onyr/.cargo/bin/evcxr_jupyter
   Installed package `evcxr_jupyter v0.14.2` (executable `evcxr_jupyter`)
warning: be sure to add `/home/onyr/.cargo/bin` to your PATH to be able to run the installed binaries

```

As asked by the result of the command, don't forget to update `~/.bashrc` with the `/home/onyr/.cargo/bin` extention to path. Once done, check the new path with `source ~/.bashrc` and `echo $PATH | grep cargo`.

2. Register the kernel with Jupyter:

<pre><div class="bg-black rounded-md mb-4"><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-sh">evcxr_jupyter --install
</code></div></div></pre>

```shell
(base) [onyr@kenzael jupyter]$ evcxr_jupyter --install
Writing /home/onyr/.local/share/jupyter/kernels/rust/kernel.json
Writing /home/onyr/.local/share/jupyter/kernels/rust/logo-32x32.png
Writing /home/onyr/.local/share/jupyter/kernels/rust/logo-64x64.png
Writing /home/onyr/.local/share/jupyter/kernels/rust/logo-LICENSE.md
Writing /home/onyr/.local/share/jupyter/kernels/rust/kernel.js
Writing /home/onyr/.local/share/jupyter/kernels/rust/lint.js
Writing /home/onyr/.local/share/jupyter/kernels/rust/lint.css
Writing /home/onyr/.local/share/jupyter/kernels/rust/lint-LICENSE
Writing /home/onyr/.local/share/jupyter/kernels/rust/version.txt
Installation complete

```

3. Run Jupyter:

<pre><div class="bg-black rounded-md mb-4"><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-sh">jupyter-notebook
</code></div></div></pre>

4. In the Jupyter Notebook interface, create a new Rust notebook and start running Rust code interactively.

Remember that you need to have Jupyter installed to use the `evcxr` kernel. You can install Jupyter using pip:

<pre><div class="p-4 overflow-y-auto"><code class="!whitespace-pre hljs language-sh">pip install jupyter
</code></div></pre>
