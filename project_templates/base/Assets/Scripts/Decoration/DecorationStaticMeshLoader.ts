class DecorationStaticMeshLoader {
    static Load(prefab_obj: GameObject, dec_ctrl:DecorationBase, list: Array<Transform>){
        if(list == null){return;}
        if(prefab_obj == null || dec_ctrl == null){return;}

        let skel_name_to_tsf = dec_ctrl.GetOrGenerateSkeltonNameToTransform();
        if(skel_name_to_tsf == null){
            Debug.Error("DecorationStaticMeshLoader: invalid skeleton.");
            return null;
        }

        // 获取当前预制体的所有MeshRenderer
        let renderer_list = prefab_obj.GetComponentsInChildren(MeshRenderer);
        if(renderer_list == null || renderer_list.length < 1){
            return;
        }

        let prefab_obj_name = prefab_obj.name;

        // 遍历所有MeshRenderer
        for (let renderer of renderer_list) {
            let renderer_obj = renderer.gameObject;
            let mount_pt_name = renderer_obj.name;
            let mtl = renderer.material;
            if (mtl.shader == null){
                mtl.shader = Shader.Find("Engine/Default");
            }

            let mount_pt_tsf = null;
            if(skel_name_to_tsf.has(mount_pt_name) == false){
                mount_pt_tsf = dec_ctrl.player.transform;
            }else{
                // 获取挂载点对应的Transform
                mount_pt_tsf = skel_name_to_tsf.get(mount_pt_name);
            }
            
            renderer_obj.name = prefab_obj_name + "_" + mount_pt_name;
            let render_tsf = renderer_obj.transform;
            // 将当前MeshRenderer的父节点设置为挂载点
            render_tsf.SetParent(mount_pt_tsf,false);
            list.push(render_tsf);
        }
    }
}