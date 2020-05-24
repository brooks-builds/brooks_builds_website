<template>
  <v-card>
    <iframe
      width="560"
      height="315"
      src="https://www.youtube.com/embed/IYO9I6AC2k8"
      frameborder="0"
      allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
      allowfullscreen
    ></iframe>
    <v-card-title>Game Programming Patterns</v-card-title>
    <v-card-text>Join me as I begin to learn game programming patterns by following the book Game Programming Patterns by Robert Nystrom. During this series I build an infinite runner game in JavaScript (using P5.js) and Rust (using GGEZ) and then refactor each game as we learn the patterns.</v-card-text>
    <v-card-actions class="d-flex justify-space-around">
      <v-btn
        :color="youtubeColor"
        dark
        href="https://youtu.be/IYO9I6AC2k8"
        target="window"
      >Watch on YouTube</v-btn>
      <v-btn
        :color="youtubeColor"
        dark
        href="https://www.youtube.com/playlist?list=PLrmY5pVcnuE9eDgLcskszIy7g0Z_hRcwk"
        target="window"
      >Playlist</v-btn>
      <v-btn
        :color="githubColor"
        dark
        href="https://github.com/brooks-builds/learning_game_design_patterns"
        target="window"
      >View Code</v-btn>
    </v-card-actions>
  </v-card>
</template>

<script>
export default {
  name: "VideoCard",
  computed: {
    youtubeColor() {
      return this.$store.state.colors.youtube;
    },
    githubColor() {
      return this.$store.state.colors.github;
    }
  }
};
</script>

<style scoped>
</style>