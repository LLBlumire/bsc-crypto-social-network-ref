<template>
  <div class="create-box">
    <v-snackbar
      v-model="createPostErrorNotice"
      :top="true"
      :timeout="3000"
      color="error"
    >
      {{createPostErrorNoticeText}}
    </v-snackbar>

    <v-card>
      <v-form @submit.prevent="createPost">
        <v-card-text>
          <v-textarea 
            v-model="postContent" 
            label="Write a Post!"
            auto-grow
            rows="1"
            prepend-icon="edit"
            clearable
          />
          <v-combobox
            class="auth-combobox"
            v-model="authReaders"
            label="Authorised Readers"
            small-chips
            clearable
            prepend-icon="visibility"
            multiple
            @change="checkAuth"
          />
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn flat color="primary" type="submit">Create Post</v-btn>
        </v-card-actions>
      </v-form>
    </v-card>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import nacl from 'tweetnacl'
import * as base64 from "@stablelib/base64";
import axios, { AxiosResponse } from "axios";
import { UserResponse } from '../model';
import { getProof } from '../auther'

export default Vue.extend({
  name: 'create-post',
  data () : {
    postContent: string,
    authReaders: string[],
    createPostErrorNoticeText: string,
  } {
    return {
      postContent: "",
      authReaders: [],
      createPostErrorNoticeText: "",
    }
  },
  methods: {
    /**
     * Create a new post
     */
    async createPost () {
      this.$emit('creating')
      let { publicKey, secretKey } = nacl.box.keyPair()

      try {
        let postContent: string = this.postContent;
        let authReaders: string[] = this.authReaders;
        this.postContent = "";
        this.authReaders = [];
        this.checkAuth();
        
        let contentNonce: Uint8Array = nacl.randomBytes(nacl.box.nonceLength);
        let message: Uint8Array = new TextEncoder().encode(postContent);
        let userSecretKey: Uint8Array = (<any>this).userSecretKey;

        // The content is encrypted to be read by the generated secret key
        let boxedContent: Uint8Array | null = nacl.box(
          message, 
          contentNonce, 
          publicKey, 
          userSecretKey
        );

        let authReaderData: UserResponse[] = await Promise.all( 
          authReaders.map(async (authUsername) => {
            try {
              return <UserResponse>(
                await axios.get('/_/user', {
                  params: {
                    username: authUsername
                  }
                })
              ).data;
            } catch {
              // Set error code
              throw new Error(`Unknown User: ${authUsername}`)
            }
          })
        )


        // For each authorised reader, encrypt the secret key
        let noaEncryptedKeys: {
          username: string,
          encryptedSecretKey: string,
          nonce: string
        }[] = authReaderData.map((authReader) => {
          let nonce = nacl.randomBytes(nacl.box.nonceLength)
          return {
            username: authReader.username,
            encryptedSecretKey: base64.encode(nacl.box(
              secretKey,
              nonce,
              base64.decode(authReader.publicKey),
              (<any>this).userSecretKey
            )),
            nonce: base64.encode(nonce)
          }
        });

        let proof: Uint8Array = await getProof(
          (<any>this).username, 
          (<any>this).userSecretKey
        )

        let publicKeyNonce: Uint8Array = nacl.randomBytes(nacl.box.nonceLength);
        let encryptedPublicKey: Uint8Array = nacl.box(
          publicKey,
          publicKeyNonce,
          (<any>this).userPublicKey,
          (<any>this).userSecretKey
        )

        if (noaEncryptedKeys.length > 0) {
          await axios.post('/_/post', {
            content: base64.encode(boxedContent),
            nonce: base64.encode(contentNonce),
            proof: base64.encode(proof),
            username: (<any>this).username,
            publicKey: base64.encode(encryptedPublicKey),
            publicKeyNonce: base64.encode(publicKeyNonce),
            noaEncryptedKeys: noaEncryptedKeys,
          })
          this.$emit('created')
        }
      } catch (e) {
        this.createPostErrorNoticeText = e.message
      }
    },

    /**
     * Check the authorised readers includes the user, and if it does not add 
     * them
     */
    checkAuth () {
      if (!this.authReaders.includes((<any>this).username)) {
        this.authReaders.push((<any>this).username)
      }
    }
  },
  computed: {
    /**
     * Boolean wrapper around error notice text for snackbar support.
     */
    createPostErrorNotice: {
      get () : boolean {
        return (<any>this).createPostErrorNoticeText !== ""
      },
      set (error) {
        if (!error) {
          (<any>this).createPostErrorNoticeText = ""
        }
      }
    },
    ...mapState({
      username: 'username',
      userSecretKey: 'secretKey',
      userPublicKey: 'publicKey',
    })
  },
  created () {
    this.checkAuth()
  }
})
</script>

<style lang="stylus">
.auth-combobox .v-input__append-inner:last-of-type
  display none
</style>