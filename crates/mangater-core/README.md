# Core Engine for Mangater - the content scrapping and management machine

core engine or orchestrator for linking up the sdk contracts to match, scrap and manage the resources.

## overall flow of orchestration

```
+--------------------------+
|   domain.match_domain()  |
|    (www.wikipedia.com)   |
+--------+-----------------+
         |
         v
+-------------------+
|   config.load()   |
|   (.env, yaml)    |
+--------+----------+
         |
         v
+-------------------+
|   resource.load() |
|   (url resource)  |
+--------+----------+
         |
         v
+----------------------------------+
|   matcher.matchPatterns()        |
|   (regex, navigation pattern)    |  <----------------+
+----------------------------------+                   |
         |                                             |
         v                                             |
+--------------------------+                           |
|   resource.load()        |                           |
|   (image, pdf resource)  |                           | (loop amongst the matches)
+--------------------------+                           |
         |                                             |
         v                                             |
+-------------------------+                            |
|   storage.persist()     |                            |
|   (file, blob storage)  |  --------------------------+
+-------------------------+
         |
         v
+-------------------+
| report generation |
+-------------------+
```
## registry concept

__registry__ is the hub that stores the implementations of supported websites; the key is the "domain" value (e.g. [https://www.wikipedia.org/](https://www.wikipedia.org/)). Under the key, a collection of traits implementations would be binded together. Hence whenever a resource url is provided, the very first thing is to scan the domain value to find a registered implementation. 

After that, the registered implementation would provide all sorts of services including ___matching___ resources (image, pdf etc) and ___persisting___ them in a managed way.

```
mangater-sdk::Domain
- match_domain() <- check if the resource url matches a Registerable struct (with all traits implementations supporting that domain)
- register_domain() <- helper fn to update the Registry on core crate

mangater-core::Registry
- the hub of supported website implementations
- work closely with the sdk crate's Domain struct
```