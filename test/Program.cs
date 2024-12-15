using System.Net;
using System.Text;

class Program
{
    static void Main(string[] args)
    {
        HttpListener listener = new HttpListener();
        listener.Prefixes.Add("http://localhost:8080/");
        listener.Start();
        Console.WriteLine("HTTP server is running on http://localhost:8080/");
        Console.WriteLine("Press Ctrl+C to stop the server.");

        while (true)
        {
            HttpListenerContext context = listener.GetContext();
            ThreadPool.QueueUserWorkItem(o => HandleRequest(context));
        }
    }

    static void HandleRequest(HttpListenerContext context)
    {
        HttpListenerRequest request = context.Request;
        HttpListenerResponse response = context.Response;

        Console.WriteLine($"Received {request.HttpMethod} request for {request.Url}");

        if (request.HttpMethod == "TRACE")
        {
            HandleTraceMethod(request, response);
        }
        else
        {
            response.StatusCode = (int)HttpStatusCode.MethodNotAllowed;
            using (StreamWriter writer = new StreamWriter(response.OutputStream))
            {
                writer.WriteLine("Method Not Allowed");
            }
        }

        response.Close();
    }

    static void HandleTraceMethod(HttpListenerRequest request, HttpListenerResponse response)
    {
        response.StatusCode = (int)HttpStatusCode.OK;
        response.ContentType = "message/http";

        using (StreamWriter writer = new StreamWriter(response.OutputStream, Encoding.UTF8))
        {
            writer.WriteLine($"{request.HttpMethod} {request.Url} HTTP/{request.ProtocolVersion}");
            foreach (string? header in request.Headers.AllKeys)
            {
                if (header == null)
                {
                    continue;
                }

                writer.WriteLine($"{header}: {request.Headers[header]}");
            }
            writer.WriteLine();
        }
    }
}
