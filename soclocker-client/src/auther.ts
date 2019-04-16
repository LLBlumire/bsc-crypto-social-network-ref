import axios from 'axios';
import { UserResponse, AuthResponse } from '@/model.ts';
import * as base64 from '@stablelib/base64';
import nacl from 'tweetnacl';

export async function getProof(username: string, localSecretKey: Uint8Array): Promise<Uint8Array> {
  let serverPublicKeyB64: string = (
    await axios.get("/_/server_public_key")
  ).data

  let auth: AuthResponse = (
    await axios.get(
      "/_/auth", 
      { 
        params: { 
          username: username 
        } 
      }
    )
  ).data

  // Decode base64 encoded data
  let encryptedToken: Uint8Array = base64.decode(auth.encryptedToken)
  let nonce: Uint8Array = base64.decode(auth.nonce)
  let serverPublicKey: Uint8Array = base64.decode(serverPublicKeyB64)
  let box: Uint8Array | null = nacl.box.open(
    encryptedToken,
    nonce,
    serverPublicKey,
    localSecretKey
  )
  if (box === null) {
    throw new Error("Cannot decrypt with given credentials.")
  }
  return box;
}