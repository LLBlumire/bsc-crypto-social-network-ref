# SocLocker

**Version 0.1.0**

SocLocker is a reference implementation of a social network specified by the 
Bachelors Project of Lucille L. Blumire (myself) entitled "Securing of Social 
Content with non-disruptive cryptography". The system aims to provide a social
network based on the philosophy that none of the content of the network is known
in its decrypted form to the server. All content stored on the network is
encrypted client side, and decrypted client side when viewed.

The idea of the system being non-disruptive is based on the fact that modern
users of the internet have ready access to keychain, password vault or password 
manager enabling them to securely store and remember their passwords across a
wide range of devices, without the need to remember an individual password for 
every website. This is a considerably more secure approach than the situation
many users of the internet place themselves in without a password manager which
is the usage of the same or procedurally similar password on every website they
hold an account for. By relying on the growing prevalence of password managers,
and using it to store a generated secret key for a public key cryptosystem, 
users can interact with a fully encrypted social network without having to go
through a lengthy sign up process of creating their own keys.

The full paper detailing this, and the ideas behind it, can be found here:

* LINK TO PAPER WILL BE PROVIDED UPON BACHELORS DEGREE COMPLETION


## License Summary

This software is licensed under [Fair Source 25](https://fair.io/). This is
intended to allow the usage of the source code in academic study, while
preserving the rights of the licensor to sell and use commercially this
software.

You are free to use (commercially or otherwise) the software with up to 25 
users. Any usage exceeding this limit requires contacting the licensor to
negotiate licensing terms, and may incur a fee subject to usage.