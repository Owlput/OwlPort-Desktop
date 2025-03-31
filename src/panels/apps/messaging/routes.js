export default{
    path:"messaging",
    component:()=>import("./Messaging.vue"),
    children:[
        {
            path:":id",
            component:()=>import("./MessagingWindow.vue")
        },
        {
            path:"",
            component:()=>import("./NoOngoingChat.vue")
        }
    ]
}