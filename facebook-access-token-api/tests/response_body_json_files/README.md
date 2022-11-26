## Files

### debug_token__200__access_token_could_not_be_decrypted.json

When input_token is wrong and access_token is correct app access token or user access token.

e.g. the wrong input_token is "EAADqHZBu6vogBAGIEufbbDLr7hSaZCNFy8r4enIYvwz7vRZC0yyyf9Q8r8Q5RdjNHfI9OXZCOMqDUKOSZAYentMtp36AXdSMW60vXJKMji0P7WdhCZAJwGgITUKURO7oOcflXsGeNnmf8QI3S0Lwh6nYQ2FTdWzg5veL3GLsuyJ0y8VgkgaqITKOZAFpeLzXDGDkP5vtxcxuSv4SG4zOtPIyeEZAZBzOXiRzyCAJZBZBTZBJBqZDZD", but the correct input_token is "EAADqHZBu6vogBAGIEufbbDLr7hSaZCNFy8r4enIYvwz7vRZC0yyyf9Q8r8Q5RdjNHfI9OXZCOMqDUKOSZAYentMtp36AXdSMW60vXJKMji0P7WdhCZAJwGgITUKURO7oOcflXsGeNnmf8QI3S0Lwh6nYQ2FTdWzg5veL3GLsuyJ0y8VgkgaqITKOZAFpeLzXDGDkP5vtxcxuSv4SG4zOtPIyeEZAZBzOXiRzyCAJZBZBTZBJBQZDZD"

### debug_token__200__cannot_get_application_info.json

When input_token is "123|abc" and access_token is correct app access token 

### debug_token__200__cannot_parse_access_token.json

When input_token is "xxxx" and access_token is correct app access token or user access token.

## debug_token__200__invalid_access_token_signature.json

When input_token is "257422819769992|abc" and access_token is correct app access token 

### debug_token__400__debug_only_access_token.json

When input_token and access_token are both "User Session Info Access Token".

E.g. debug "Session Info Access Token" in https://developers.facebook.com/tools/debug/accesstoken .

### debug_token__400__must_provide_an_app_access_token_or_a_user_access_token.json

When missing access_token query parameter.

### debug_token__user_access_token_1.json

When input_token and access_token are both "Short-Lived User Access Token".

Maybe has metadata if from https://developers.facebook.com/tools/explorer/

```
"metadata": {
    "auth_type": "rerequest"
},
```

### debug_token__user_access_token_2.json

When input_token and access_token are both "Long-Lived User Access Token".

When input_token is "User Session Info Access Token" and access_token is "Long-Lived User Access Token".

### debug_token__user_access_token_3.json

When input_token and access_token are both expired "User Access Token".

### debug_token__user_access_token_4.json

When input_token and access_token are both invalidated "User Access Token".
