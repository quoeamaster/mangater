## SDK / interface for Mangater - the content scrapping and management machine

acting as the sdk / interface for mangater to work; this is the heart of the ecosystem where contributors can supply implementation details on how to match resource links, download them, search for the next resource, persist and generate a scrap summary.

However, mangater is not a crawling engine, hence it would only serve to locate resources within a logic page or chapter. For most cases, it will not crawl out of the boundary.

## possible layouts of websites containing the resources

### a. everything is on the landing page (e.g. gallery pages)
- all resources (images, pdf etc) are all linked on the landing page
- matching logic can source all the resources on the landing page; no further page navigation required

__traits to implement__
```
domain.rs
    - match(domain:String)

matcher.rs 
    - matchPatterns([ PatternAndType { pattern:String, type:Enum } ])

storage.rs
    - persist()
```
### b. langing page shows only the 1st resource, links to the next resource is provided on the page
- only 1 resource available per page
- navigation of pages required (but not crawling over the logical boundary of a chapter or unit)
- matching logic can source the only 1 resource on the current page
- matching logic have to source for the next pagination resource / url

__traits to implement__
```
domain.rs
    - match(domain:String)

matcher.rs 
    - matchPatterns([ PatternAndType { pattern:String, type:Enum } ])

storage.rs
    - persist()
```

