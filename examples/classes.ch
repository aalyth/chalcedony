
class Header:
    header: str
    value: str

    fn new(header: str, value: str) -> Header:
        return Header {header, value}

    fn display(self) -> str:
        return self.header + ": " + self.value

class HttpRequestMock:
    method: str
    url: str
    headers: [Header] 
    body: str

    fn new() -> HttpRequestMock:
        return HttpRequestMock {
            method: "",
            url: "",
            headers: List::new(),
            body: ""
        }

    fn set_method!(self, method: str) -> HttpRequestMock:
        if self.method != "":
            throw "setting an already set method"

        self.method = method
        return self

    fn set_url!(self, url: str) -> HttpRequestMock:
        if self.url != "":
            throw "setting an already set url"
            
        self.url = url 
        return self

    fn set_body!(self, body: str) -> HttpRequestMock:
        if self.body != "":
            throw "setting an already set body"
            
        self.body = body 
        return self

    fn add_header!(self, header: Header) -> HttpRequestMock:
        for h in self.headers:
            if h.header == header.header:
                throw "header " + header.header + " already exists"
        
        self.headers.push_back(header)
        return self

    fn send(self):
        print(self.method + " " + self.url)
        for header in self.headers:
            print(header.display())
        print(self.body)


if __name__ == "__main__":
    HttpRequestMock::new() \
        .set_method!("GET") \
        .set_url!("elsys-bg.org") \
        .add_header!(Header::new("Bearer", "<some-token>")) \
        .add_header!(Header::new("Expires", "1716390850")) \
        .set_body!("Hello to Elsys") \
        .send()
