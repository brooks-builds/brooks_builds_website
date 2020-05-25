<template>
  <v-app>
    <Navigation />

    <v-content>
      <v-container>
        <v-row>
          <v-col cols="4">
            <v-row>
              <v-col>
                <About />
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <Schedule />
              </v-col>
            </v-row>
          </v-col>
          <v-spacer></v-spacer>
          <v-col cols="6" class="mt-7">
            <PremiereVideo />
          </v-col>
        </v-row>
        <v-row>
          <RecentVideos />
        </v-row>
        <v-row>
          <ChatCommands />
        </v-row>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
import Navigation from "./components/Navigation";
import About from "./components/About";
import Schedule from "./components/Schedule";
import RecentVideos from "./components/RecentVideos";
import PremiereVideo from "./components/PremiereVideo";
import ChatCommands from "./components/ChatCommands";

export default {
  name: "App",

  components: {
    Navigation,
    About,
    Schedule,
    RecentVideos,
    PremiereVideo,
    ChatCommands,
  },

  mounted() {
    this.$store.dispatch("fetchAllChatCommands");
  },
};
</script>
