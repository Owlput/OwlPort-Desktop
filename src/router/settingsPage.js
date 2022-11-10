export default  {
    path:'/settings',
    component:()=>import("../pages/settings/Settings.vue"),
    children:[
        {
            path:"certificates",
            component:()=>import("../pages/settings/Certificates.vue")
        },
        {
            path:"general",
            component:()=>import("../pages/settings/General.vue")
        }
    ]
}