# Cotyledon

I want this to be an stateless microservice that lets a person cultivate a
digital garden. Cotyledon refers to the first leaf(s) a seedling produces:
<https://en.wikipedia.org/wiki/Cotyledon>.

## Design Decisions

### Stateless

This API will be stateless so the client will have to store session data. I think
this will achieve the most flexible and reliable/secure system while still
maintaining statelessness.
