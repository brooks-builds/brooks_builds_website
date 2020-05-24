<template>
  <VideoCard
    videoSource="https://www.youtube.com/embed/IYO9I6AC2k8"
    title="Game Programming Patterns"
    body="Join me as I begin to learn game programming patterns by following the book Game Programming Patterns by Robert Nystrom. During this series I build an infinite runner game in JavaScript (using P5.js) and Rust (using GGEZ) and then refactor each game as we learn the patterns."
    directLink="https://youtu.be/IYO9I6AC2k8"
    playlistLink="https://www.youtube.com/playlist?list=PLrmY5pVcnuE9eDgLcskszIy7g0Z_hRcwk"
    codeLink="https://github.com/brooks-builds/learning_game_design_patterns"
  />
</template>

<script>
import VideoCard from "./VideoCard";

export default {
  components: {
    VideoCard
  }
};
</script>

<style scoped>
</style>