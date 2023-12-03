<template>
    <div class="container">
        <div class="menu">
            <button @click="mode = mode == 'create' ? 'view' : 'create'" :disabled="mode == 'edit'">Добавить</button>
            <button @click="clearData()">Удалить все</button>
        </div>
        {{ mode }}
        <div v-if="mode == 'edit' || mode == 'create'" class="editor">
            <div class="error">{{ error }}</div>
            <div class="title">
                <label for="title">Вопрос: </label>
                <input type="text" v-model="currect.title">
            </div>
            <div class="answers">
                <div><label>1</label>
                    <input v-model="currect.answers[1].text"><input type="radio" @change="currect.success = 1"
                        :checked="currect.success == 1"  />
                </div>
                <div> <label>2</label>
                    <input v-model="currect.answers[2].text"><input type="radio" @change="currect.success = 2"
                        :checked="currect.success == 2"  />
                </div>
                <div>
                    <label>3</label>
                    <input v-model="currect.answers[3].text"><input type="radio" @change="currect.success = 3"
                        :checked="currect.success == 3"  />
                </div>
                <div>
                    <label>4</label>
                    <input v-model="currect.answers[4].text"><input type="radio" @change="currect.success = 4"
                        :checked="currect.success == 4"  />
                </div>
            </div>
            <button @click="addData()">Сохранить</button>
            <button @click="OffEdit()" v-if="mode == 'edit'">Отмена</button>
        </div>
        <div class="list">
            <div class="card" v-for="el in data.data">
                <div class="card__title">{{ el.title }}</div>
                <label class="card__answer" v-for="answer in el.answers">

                    <span class="card__answer-radio" :class="{
                        select: answer.num === el.success
                    }"></span>
                    <span class="card__answer-text">{{ answer.text }}</span>
                </label>
                <button @click="deleteData(el.id)">удалить</button>
                <button @click="editMode(el.id)">изменить</button>
                <button v-if="el.id != 0" @click="  upArr(data.data, el.id)">вверх</button>
                <button v-if="el.id !== data.data.length - 1" @click=" downArr(data.data, el.id)">вниз</button>

            </div>


        </div>
    </div>
</template>

<style scoped>
.container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-content: center;

}

.select {
    background-color: #bbb;
}

.editor {
    display: flex;
    flex-direction: column;
    border: 1px solid black;
    padding: 10px;
}

.list {
    display: flex;
    flex-direction: column;
    align-items: center;
}

.card {
    width: 400px;
    border: 1px solid #ddd;
    border-radius: 4px;
    padding: 16px;
}

.card__title {
    font-size: 18px;
    font-weight: bold;
    margin-bottom: 16px;
}

.card__answer {
    display: block;
    margin-bottom: 8px;
}

.card__answer:last-child {
    margin-bottom: 0;
}

.card__answer-radio {
    display: inline-block;
    width: 18px;
    height: 18px;
    border: 1px solid #bbb;
    border-radius: 50%;
    margin-right: 8px;
    vertical-align: middle;
}

.card__answer-text {
    display: inline-block;
    vertical-align: middle;
}
</style>


<script setup>
import { reactive, ref, toRaw, onMounted, watch } from "vue"
import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';
const data = reactive({
    data: []
})
const error = ref()
const mode = ref("view")
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
async function clearData() {
    data.data = []
    saveData()
    loadData()
    addId()
}
async function OffEdit() {
    mode.value = 'view'
    currect.id = -1
    currect.title = "";
    currect.answers = {
        1: { num: 1, text: "" },
        2: { num: 2, text: "" },
        3: { num: 3, text: "" },
        4: { num: 4, text: "" },
    }
    currect.success = null
}


async function editMode(id) {
    mode.value = 'edit'
    currect.id = data.data[id].id
    currect.title = data.data[id].title
    currect.answers = data.data[id].answers
    currect.success = data.data[id].success
}
async function addData() {
    
    if (currect.title === '' || (currect.answers[1].text === '' || currect.answers[2].text === '' || currect.answers[3].text === '' || currect.answers[4].text === '') || currect.success === null) {
        error.value = 'Заполните все поля и выберите верный вариант ответа'
        return
    }
    error.value = ""
    if (mode.value === 'edit') {
        data.data[currect.id] = {
        id: currect.id,
        title: currect.title,
        answers: currect.answers,
        success: currect.success
    }
    }
    else {
        data.data.push({
        title: currect.title,
        answers: currect.answers,
        success: currect.success
    })
    }
    addId()
    await saveData()
}
async function upArr(arr, id) {
    const index = arr.findIndex(item => item.id === id);

    if (index > 0) {
        // Меняем местами с предыдущим элементом
        [arr[index], arr[index - 1]] = [arr[index - 1], arr[index]];
    }

    data.data = arr;
    addId()
    await saveData()
}
async function deleteData(id) {
    data.data.splice(id, 1);
    addId()
    saveData()
    await saveData()
}
async function downArr(arr, id) {
    const index = arr.findIndex(item => item.id === id);

    if (index < arr.length - 1) {
        // Меняем местами со следующим элементом 
        [arr[index], arr[index + 1]] = [arr[index + 1], arr[index]];
    }
    data.data = arr;
    addId()
    await saveData()
}
async function saveData() {

    const jsonData = JSON.stringify(toRaw(data.data));

    await writeTextFile('.\\data.json', jsonData);
    currect.id = -1
    currect.title = "";
    currect.answers = {
        1: { num: 1, text: "" },
        2: { num: 2, text: "" },
        3: { num: 3, text: "" },
        4: { num: 4, text: "" },
    }
    currect.success = null
}

onMounted(async () => {
    data.data = await loadData()
    addId()
})


const currect = reactive({
    id: -1,
    title: "",
    answers: {
        1: { num: 1, text: "" },
        2: { num: 2, text: "" },
        3: { num: 3, text: "" },
        4: { num: 4, text: "" },
    },
    success: null
})


</script>

