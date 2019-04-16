<template>
  <div class="post-box pa-2">
    <v-card :title="post.post.TimePosted">
      <v-card-title primary-title>
        <div>
          <div class="headline">{{post.post.username}}</div>
          <div class="caption">Posted {{timestamp}}</div>
        </div>
      </v-card-title>
      <v-card-text>
        <div class="body-1">
          {{content}}
          <v-btn
            v-if="isOwnPost"
            flat
            icon
            color="secondary"
            @click="editMode = !editMode"
          >
            <v-icon dark>edit</v-icon>
          </v-btn>
          <v-expand-transition>
            <form @submit.prevent="submitEdit" v-if="editMode">
              <v-textarea 
                v-model="editContent" 
                label="New Post Content"
                auto-grow
                rows="1"
                prepend-icon="edit"
                clearable
              />
              <div>
                <v-btn 
                  flat
                  color="error"
                  @click="toggleEdit"
                  :disabled="processEdit"
                >
                  Cancel
                </v-btn>
                <v-btn
                  flat
                  color="primary"
                  type="submit"
                  :loading="processEdit"
                >
                  Submit Edit
                </v-btn>
              </div>
            </form>
          </v-expand-transition>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-chip 
          color="secondary"
          dark
          disabled
          label
          outline
          small
          v-for="reader in post.allReaders"
          :key="reader"
        >
          {{reader}}
        </v-chip>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import nacl from 'tweetnacl';
import axios from 'axios'
import * as base64 from '@stablelib/base64';
import { getProof } from '../auther'

export default Vue.extend({
  name: 'post-box',
  props: {
    post: {
      type: Object,
      required: true
    }
  },
  data (): {
    editMode: boolean,
    editContent: string,
    processEdit: boolean,
  } {
    return {
      editMode: false,
      editContent: "",
      processEdit: false,
    }
  },
  computed: {
    ...mapState({
      isLoggedIn: 'isLoggedIn',
      userPublicKey: 'publicKey', 
      userSecretKey: 'secretKey',
      username: 'username'
    }),
    isOwnPost(): boolean {
      return this.post.post.username == (<any>this).username
    },
    timestamp(): string {
      return new Date(this.post.post.timePosted).toString()
    },
    content(): string {
      if (!(<any>this).isLoggedIn) {
        return "login error"
      }
      let postNonce: Uint8Array = base64.decode(this.post.post.nonce)
      let postMsg: Uint8Array = base64.decode(this.post.post.encryptedContent)
      let posterPublicKey: Uint8Array = base64.decode(this.post.post.publicKey)
      
      let encNonce: Uint8Array = base64.decode(this.post.nonce)
      let encMsg: Uint8Array = base64.decode(this.post.encryptedSecretKey)
      let userSecretKey: Uint8Array = (<any>this).userSecretKey

      let secretKey: Uint8Array = nacl.box.open(
        encMsg,
        encNonce,
        posterPublicKey,
        userSecretKey
      )!

      let post: Uint8Array = nacl.box.open(
        postMsg,
        postNonce,
        posterPublicKey,
        secretKey
      )!
      
      return new TextDecoder().decode(post);

    },
  },
  methods: {
    toggleEdit(): void {
      this.editMode = !this.editMode
    },
    async submitEdit() {
      try {
        this.processEdit = true
        let encryptedPublicKey: Uint8Array = base64.decode(
          this.post.post.encryptedPublicKey
        )
        let nonce: Uint8Array = base64.decode(
          this.post.post.encryptedPublicKeyNonce
        )

        let publicKey: Uint8Array | null = nacl.box.open(
          encryptedPublicKey, 
          nonce, 
          (<any>this).userPublicKey,
          (<any>this).userSecretKey
        )

        if (publicKey === null) {
          throw new Error('Unable to decrypt public key')
        }

        let proof: Uint8Array = await getProof(
          (<any>this).username,
          (<any>this).userSecretKey
        );


        let newContent: Uint8Array = new TextEncoder().encode(this.editContent)
        let newNonce: Uint8Array = nacl.randomBytes(nacl.box.nonceLength)
        let userSecretKey: Uint8Array = (<any>this).userSecretKey;

        let newEncryptedContent: Uint8Array = nacl.box(
          newContent,
          newNonce,
          publicKey,
          userSecretKey,
        )

        await axios.put("/_/post", {
          postId: this.post.post.postId,
          proof: base64.encode(proof),
          newContent: base64.encode(newEncryptedContent),
          newNonce: base64.encode(newNonce)
        })
        this.toggleEdit()
      } catch (e) {
        console.log(`Error in Post edit: ${e}`)
      } finally {
        this.processEdit = false
        this.$emit('edited')
      }
    }
  },
  created (): void {
    this.editContent = this.content;
  }
})
</script>

