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

<blockquote>
PS. the engine struct would hold a Registry instance; ultimately the CLI would need to pass this Registry reference to the sdk::Domain implementation to get themselves registered
</blockquote>

```
+========================+
|| registration starts  ||
+========+===============+
         |
         v
+-----------------------+
|   engine.registry()   |  <- get access to the sdk::traits::Registry implementation
+--------+--------------+
         |
         v
+-------------------------------+
|   registr.add_to_registry()   |  <- pass an implementation of sdk::traits::Domain to the registry instance for registration
+--------+----------------------+
         |
         v
+------------------------------+
|   domain.register_domain()   |  <- the Domain implementation would call its register_domain against the Registry instance
+--------+---------------------+
         |
         v
+=======================+
||  registration done  ||  <- loop the process until all Registerable(s) are registered
+=======================+
```
## configuration concept

Core engine will support 2 ways to load configuration
- env variables loaded through .env file (can customize filename and path) OR
- json config file (can customize filename and path)

These 2 approaches should cover most of the configuration use cases; however if a custom approach is required. It is possible to provide an implementation of the sdk::traits::config trait. The implementation will be fully responsible for the following:
- config data loading
- transform the read config content into actual struct for access
- providing access to the config through a key

such examples could be:
- config data stored in databases
- config data stored in a midde tier technology such as [Redis](https://redis.io/) / [Elasticsearch](https://www.elastic.co/)
- config data stored in [yaml](https://en.wikipedia.org/wiki/YAML) / [toml](https://toml.io/en/) (official support config file is json ONLY at this moment)

