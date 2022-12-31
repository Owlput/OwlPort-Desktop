export default  {
    path:'/settings',
    component:()=>import("../layouts/Settings.vue"),
    children:[
        {
            path:"certs-and-keys",
            component:()=>import("../pages/settings/CertsAndKeys.vue")
        },
        {
          path:"general",
          component:()=>import("../pages/settings/General.vue")
      },
        {
            path:"",
            component:()=>import("../pages/settings/General.vue")
        }
    ]
}
