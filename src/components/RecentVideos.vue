<template>
  <v-container>
    <v-row class="justify-center">
      <h1>Recent Videos</h1>
    </v-row>
    <v-row>
      <v-col cols="4">
        <VideoCard
          videoSource="https://www.youtube.com/embed/Jj6NmCRVktU"
          title="Nginx and Node.js by Kim Schlesinger - Node.js meetup May 2020"
          body="Kim Schlesinger talks at the Denver Node.js meetup about nginx and Node.js"
          directlink="https://www.youtube.com/watch?v=Jj6NmCRVktU"
          playlistLink="https://www.youtube.com/playlist?list=PLrmY5pVcnuE9XsFJVLTbv_08V58xNk7zd"
          meetupLink="https://www.meetup.com/Node-js-Denver-Boulder/events/ktpcnrybchbcc/"
        />
      </v-col>
      <v-col cols="4">
        <VideoCard
          videoSource="https://www.youtube.com/embed/ljp7zZ6byyU"
          title="Game Programming Patterns part 5.2 - (Rust) Command"
          body="We continue to work on implementing the Command pattern into the Rust infinite runner"
          directlink="https://www.youtube.com/watch?v=ljp7zZ6byyU"
          playlistLink="https://www.youtube.com/playlist?list=PLrmY5pVcnuE9eDgLcskszIy7g0Z_hRcwk"
          codeLink="https://github.com/brooks-builds/learning_game_design_patterns"
        />
      </v-col>
      <v-col cols="4">
        <VideoCard
          videoSource="https://www.youtube.com/embed/ACFSC77nFu0"
          title="Game Programming Patterns part 5.3 - (Rust) Command"
          body="I finish implementing  the Command game programming pattern into our GGEZ infinite runner."
          directlink="https://www.youtube.com/watch?v=ACFSC77nFu0"
          playlistLink="https://www.youtube.com/playlist?list=PLrmY5pVcnuE9eDgLcskszIy7g0Z_hRcwk"
          codeLink="https://github.com/brooks-builds/learning_game_design_patterns"
        />
      </v-col>
    </v-row>
    <v-row>
      <v-col>
        <v-btn
          :color="youtubeColor"
          block
          dark
          href="https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ"
          target="window"
        >Watch More on YouTube</v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import VideoCard from "./VideoCard";

export default {
  name: "RecentVideos",
  components: {
    VideoCard
  },
  computed: {
    youtubeColor() {
      return this.$store.state.colors.youtube;
    }
  }
};
</script>

<style scoped>
</style>