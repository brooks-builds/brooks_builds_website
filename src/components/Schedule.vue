<template>
    <v-container>
        <v-row class="justify-center">
            <h1>Schedule</h1>
        </v-row>
        <v-row>
            <p>Every weekday <span class="bold">7am - 8:15am</span> Mountain Time</p>
        </v-row>
        <v-row>
            <v-col>
                <v-btn block color="#9147ff" dark href="https://www.twitch.tv/brookzerker" target="window">Follow on Twitch</v-btn>
            </v-col>
        </v-row>
        <v-row>
            <v-col>
                <v-btn block color="#ff0000" dark href="https://www.youtube.com/channel/UCT1-XRVnJA-wws2bfbLbFcQ" target="window">Subscribe on YouTube</v-btn>
            </v-col>
        </v-row>
        <v-row>
            <v-col>
                <v-btn block color="#1da1f2" dark href="https://www.twitter.com/brooks_patton" target="window">Follow on Twitter</v-btn>
            </v-col>
        </v-row>
        <v-row>
            <v-col>
                <v-btn block color="#24292e" dark href="https://www.github.com/brooks-builds" target="window">See the Code on GitHub</v-btn>
            </v-col>
        </v-row>
    </v-container>
</template>

<script>
export default {
    name: 'Schedule',
}
</script>

<style scoped>
.bold {
    font-weight: bold;
}
</style>