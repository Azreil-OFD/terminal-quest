<template>
    <div v-if="mode == 'error'">Нету вопросов</div>
    <section id="start_page" v-if="mode == 'start'">
        <button @click="startGame()" >Старт</button>
    </section>
    <section id="game_page" v-if="mode == 'game'">
        <p>{{ data.data[quest].title }}</p>
        <button @click="addAnswer(data.data[quest].answers[1].num)" >{{data.data[quest].answers[1].text}}</button>
        <button @click="addAnswer(data.data[quest].answers[2].num)" >{{data.data[quest].answers[2].text}}</button>
        <button @click="addAnswer(data.data[quest].answers[3].num)" >{{data.data[quest].answers[3].text}}</button>
        <button @click="addAnswer(data.data[quest].answers[4].num)" >{{data.data[quest].answers[4].text}}</button>

    </section>

    <section id="start_page" v-if="mode == 'finish'">
        <p>{{ score }}/{{ answers.length }}</p>
        <button @click="mode = 'start'" >На главную</button>
    </section>
</template>

<style scoped>
</style>


<script setup>
import { reactive, ref, toRaw, onMounted, watch, computed } from "vue"
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
const data = reactive({
    data: []
})
const mode = ref("start")
const quest = ref(0)
const answers = ref([])
const score = ref(0)

const startGame = () => {
    quest.value = 0
    mode.value = 'game'
    score.value = 0
    answers.value = []
}
const addAnswer=(num) =>{
    answers.value.push(num)
    quest.value += 1
    if (quest.value == data.data.length) {
        mode.value = "finish"
        let count = 0;
        data.data.forEach(element => {
            console.log(element.success , answers.value[count])
            if(element.success == answers.value[count] ) {
                score.value += 1
            }
            count += 1;
        });

    }
}
async function loadData() {
    try {
        const jsonData = await readTextFile('.\\data.json');

        return JSON.parse(jsonData);
    } catch {

        return [];
    }
}
function addId() {
    console.log()
    for (let index = 0; index < data.data.length; index++) {
        data.data[index].id = index
    }


}
onMounted(async () => {
    data.data = await loadData()
    if (data.data == []) {
        mode = "error"
    }
})
</script>

