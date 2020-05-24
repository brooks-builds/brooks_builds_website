<template>
  <v-card>
    <iframe
      width="100%"
      :src="videoSource"
      frameborder="0"
      allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
      allowfullscreen
    ></iframe>
    <v-card-title>{{ title }}</v-card-title>
    <v-card-text>{{ body }}</v-card-text>
    <v-card-actions class="d-flex justify-space-around">
      <v-btn :color="youtubeColor" dark :href="directLink" target="window">Watch on YouTube</v-btn>
      <v-btn :color="youtubeColor" dark :href="playlistLink" target="window">Playlist</v-btn>
      <v-btn :color="githubColor" dark :href="codeLink" target="window" v-if="codeLink">Code</v-btn>
      <v-btn :color="meetupColor" dark :href="meetupLink" target="window" v-if="meetupLink">Meetup</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  name: "VideoCard",
  props: {
    videoSource: String,
    title: String,
    body: String,
    directLink: String,
    playlistLink: String,
    codeLink: String,
    meetupLink: String
  },
  computed: {
    youtubeColor() {
      return this.$store.state.colors.youtube;
    },
    githubColor() {
      return this.$store.state.colors.github;
    },
    meetupColor() {
      return this.$store.state.colors.meetup;
    }
  }
};
</script>

<style scoped>
iframe {
  height: 20rem;
}
</style>