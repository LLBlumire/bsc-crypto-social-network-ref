<template>
  <div class="feed">
    <v-snackbar
      v-model="loadPostsErrorNotice"
      :top="true"
      :timeout="3000"
      color="error"
    >
      {{loadPostsErrorNoticeText}}
    </v-snackbar>

    <create-post @created="singleUpdatePosts"/>
    <v-layout justify-center class="mt-4">
      <v-pagination 
        v-model="page"
        :length="pages"
        :total-visible="7"
        @input="singleUpdatePosts"
      />
    </v-layout>
    <post-box 
      v-for="post in posts" 
      :key="post.post.postId" 
      :post="post" 
      class="mt-4"
      @edited="singleUpdatePosts"  
    />
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { mapState } from 'vuex'
import CreatePost from '@/components/CreatePost.vue'
import PostBox from '@/components/PostBox.vue'
import axios from 'axios'

export default Vue.extend({
  name: 'feed',
  components: {
    CreatePost,
    PostBox
  },
  data (): {
    page: number,
    loadPostsErrorNoticeText: string,
    posts: {
      post: {
        encryptedContent: string,
        nonce: string,
        username: string,
        publicKey: string,
        postId: number,
        timePosted: string,
        encryptedPublicKey: string,
        encryptedPublicKeyNonce: string,
      },
      nonce: string,
      allReaders: string[],
    }[]
    pages: number,
  } {
    return {
      loadPostsErrorNoticeText: "",
      page: 1,
      posts: [],
      pages: 0,
    }
  },
  computed: {
    ...mapState(['isLoggedIn', 'username']),
    loadPostsErrorNotice: {
      get () : boolean {
        return (<any>this).loadPostsErrorNoticeText !== ""
      },
      set (error) {
        if (!error) {
          (<any>this).loadPostsErrorNoticeText = ""
        }
      }
    },

  },
  created () {
    if (!((<any>this).isLoggedIn)) {
      this.$router.push({ name: 'public' })
    }
    this.updatePosts()
  },
  methods: {
    async singleUpdatePosts() {
      if ((<any>this).isLoggedIn) {
        try {
          let data = (
            await axios.get('/_/noa', {
              params: {
                username: (<any>this).username,
                skip: this.page - 1,
              }
            })
          ).data
          data.noas.sort((a: any, b: any) => {
            return -(new Date(a.post.timePosted).getTime() - new Date(b.post.timePosted).getTime())
          })
          this.posts = data.noas
          this.pages = data.pages
        } catch {
          this.loadPostsErrorNoticeText = "Unable to load posts"
        }
      }
    },
    updatePosts() {
      this.singleUpdatePosts().then(() => {
        setTimeout(this.updatePosts, 30000)
      })
    }
  }
})
</script>

