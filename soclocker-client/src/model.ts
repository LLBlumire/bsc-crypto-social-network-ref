/**
 * Represents the response of a GET request to the `auth` endpoint.
 */
export interface AuthResponse {
  encryptedToken: string;
  nonce: string;
}

/**
 * Represents the response of a GET request to the `user` endpoint.
 */
export interface UserResponse {
  id: number;
  publicKey: string;
  username: string;
}
