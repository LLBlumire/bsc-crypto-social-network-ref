<template>
  <div class="public-view">
    <v-card class="pa-3">
      <v-card-title primary-title>
        <h1 class="display-2">
          Welcome to SocLocker!
        </h1>
      </v-card-title>
      <v-card-text>
        <p>
          SocLocker is a prototype fully encrpted social network. All content
          hosted on the website is end to end encrypted, with the server having
          no knowledge of the means of decrypting it. This means that content
          put on SocLocker is 100% secure and free from prying eyes.
        </p>

        <h2 class="headline">Here's an overview of how it works!</h2>

        <v-timeline dense class="ma-2">
          <v-timeline-item
            v-for="entry in timeline"
            :key="entry.title"
            :color="entry.color"
            :icon="entry.lock ? 'lock' : 'lock_open'"
            fill-dot
            large
          >
            <v-card>
              <v-card-title :class="entry.color">
                <v-icon dark size="42" class="mr-3">{{ entry.icon }}</v-icon>
                <h2
                  class="headline white--text font-weight-light"
                  v-html="entry.title"
                >
                  {{ entry.title }}
                </h2>
              </v-card-title>
              <v-card-text>
                <p v-html="entry.content" />
              </v-card-text>
            </v-card>
          </v-timeline-item>
        </v-timeline>
      </v-card-text>
      <v-card-actions>
        <v-btn large block color="primary" @click="$emit('register')">
          Register Now!
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  name: "public-view",
  data(): {
    timeline: {
      color: String;
      lock: Boolean;
      icon: String;
      title: String;
      content: String;
    }[];
  } {
    return {
      timeline: [
        {
          color: "info",
          lock: false,
          icon: "edit",
          title: "Content Creation",
          content:
            "Safe in the knowledge that your content will only be " +
            "viewed by those you permit to, you create exactly the kind of " +
            "that you want to create, without concern for those you'd rather " +
            "not see it ever having access to it."
        },
        {
          color: "error",
          lock: true,
          icon: "visibility_off",
          title: "Browser Side Encryption",
          content:
            "Your web browser takes all the steps requried to encrypt " +
            "the content you have created, without it ever leaving your " +
            "computer. It does this using a combination of " +
            "<a href='https://cr.yp.to/snuffle/salsafamily-20071225.pdf'>" +
            "Salsa20</a> " +
            "and " +
            "<a href='https://tools.ietf.org/html/rfc8439'>Poly1305</a> " +
            "to ensure that your content is as secure as it can be!"
        },
        {
          color: "secondary",
          lock: true,
          icon: "cloud_upload",
          title: "Publishing",
          content:
            "The content you've encrypted is sent to the server, " +
            "along with the list of people you wish to have access to the " +
            "content. You're free to update and modify this list at any time, " +
            "but you can only revoke access on new versions of the content, as " +
            "once a user has access to content, that access is immutable. It's " +
            "simply outside of our control due to the encrypted nature of the " +
            "service."
        },
        {
          color: "warning",
          lock: true,
          icon: "find_in_page",
          title: "Content Accessed",
          content:
            "When a user attempts to access your content, they are " +
            "given a copy of the fully encrypted version that is stored on the " +
            "server. This copy is completely useless to them unless they are " +
            "one of the people you have granted access to the content on, for " +
            "only those people know the secure method to decrypt the content."
        },
        {
          color: "success",
          lock: false,
          icon: "visibility",
          title: "Browser Side Decryption",
          content:
            "Having acquired a copy of the encrypted content, <b>if " +
            "and only if</b> it is one of the users you have granted access " +
            "to, their web browser will decrypt the content. This leaves your " +
            "original content, verified and unmodified, visible to them."
        }
      ]
    };
  }
});
</script>

<style lang="stylus">
.public-view .v-icon
  user-select none
</style>
