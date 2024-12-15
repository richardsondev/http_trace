# http_trace

`http_trace` is a Rust application that sends HTTP TRACE requests to a specified URL. It validates the URL and returns the TRACE response, which can be useful for testing servers that support the HTTP TRACE method or for debugging request flows.

## What Does HTTP TRACE Return?

When you send an HTTP TRACE request to a server that supports it, the server should return the exact request message it received, including all headers. This lets you see how intermediaries—such as proxies and load balancers—may have modified the request before it reached the server.

**Note:** Many servers and proxies disable or block TRACE for security reasons. If it is enabled, a typical response might look like the below example.

## Example Usage

### Running the Binary
```bash
http_trace.exe http://example.com/
```

### Example Response
```
HTTP/1.1 200 OK
Date: Sat, 14 Dec 2024 12:00:00 GMT
Server: Apache/2.4.41 (Ubuntu)
Content-Type: message/http
Content-Length: 162

TRACE / HTTP/1.1
Host: example.com
Accept: */*
```

In this example, the server responds with `200 OK` and echoes the request line and headers it received. The application exits with code 0.

## Getting Started

### Prebuilt Binaries

You can download prebuilt binaries for various platforms from the [Releases](https://github.com/richardsondev/http_trace/releases) page. Select the binary that matches your operating system and architecture, then place it in your desired directory.

### Building from Source

If you prefer to build the application from source, you will need Rust and Cargo installed. First, clone the repository and navigate to its root directory:

```bash
git clone https://github.com/richardsondev/http_trace.git
cd http_trace
```

Then, build the application:

```bash
cargo build --release
```

This creates an optimized binary in the `target/release` directory.

### Running the Application

If you are using a downloaded binary, ensure it is executable and run:

```bash
./http_trace <url>
```

If you built from source, you can run the optimized binary:

```bash
./target/release/http_trace <url>
```

Replace `<url>` with the URL you want to send the TRACE request to.

**Important:** If no URL is provided, or if the URL is invalid, the program will exit with an error message and a nonzero code.

### Exit Codes

- **0**: The TRACE request succeeded.
- **1**: The TRACE request failed, the URL was invalid, or no URL was provided.

## Contributing

Contributions are welcome! Feel free to submit a pull request. For major changes, consider opening an issue first to discuss what you would like to modify.

## License

This project is licensed under the [MIT License](LICENSE).
