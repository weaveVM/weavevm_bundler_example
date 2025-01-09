# weavevm_bundler_example

The [WeaveVM Bundler](https://github.com/weavevm/bundler) is an easy way to post data to [WeaveVM](https://wvm.dev) from inside a Rust program. Bundles can contain multiple transactions while incurring only one base fee. Transactions inside bundles are also processed in parallel, meaning they are independent of the network’s block time.

This example repo demonstrates how to post the full contents of 3 books onchain using WeaveVM Bundler. The books to post are located in the `data` directory.

[Read the full tutorial blog post here](https://blog.wvm.dev/bundler-tutorial/)
