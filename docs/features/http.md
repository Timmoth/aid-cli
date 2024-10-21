## aid http
```
  aid http req    Make a HTTP request
            -m, --method <METHOD>  Specify the HTTP method (e.g., GET, POST).
            -u, --url <URL>        Specify the URL for the HTTP request.
            -c, --config <CONFIG>  Path to a configuration file for the request. Specify: method, url, body, headers in json format.
            -o, --output <OUTPUT>  If specified saves http response body to a file at the given path.
            
  aid http serve  Start a HTTP server (GET: 0.0.0.0:80 -> 'Hello, World!')
            -p, --port <PORT>  Specify the port for the HTTP server (default is 80). [default: 80]
```