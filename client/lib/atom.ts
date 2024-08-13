import { atom } from "jotai";


const loginDataAtom = atom({
    email: "",
    message: "",
    status: "",
    token: "",
    user_reference: null,
    username: ""
})

export {
    loginDataAtom
}