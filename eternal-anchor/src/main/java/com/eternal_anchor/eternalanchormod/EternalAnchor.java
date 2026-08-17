package com.eternal_anchor.eternalanchormod;

import com.mojang.logging.LogUtils;
import org.slf4j.Logger;

import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.CreativeModeTabs;
import net.minecraft.world.level.block.SoundType;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.material.MapColor;
import net.neoforged.bus.api.IEventBus;
import net.neoforged.fml.ModContainer;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.event.BuildCreativeModeTabContentsEvent;
import net.neoforged.neoforge.registries.DeferredBlock;
import net.neoforged.neoforge.registries.DeferredItem;
import net.neoforged.neoforge.registries.DeferredRegister;

@Mod(EternalAnchor.MODID)
public class EternalAnchor {
    public static final String MODID = "eternalanchor";
    public static final Logger LOGGER = LogUtils.getLogger();

    public static final DeferredRegister.Blocks BLOCKS = DeferredRegister.createBlocks(MODID);
    public static final DeferredRegister.Items ITEMS = DeferredRegister.createItems(MODID);

    public static final DeferredBlock<EternalAnchorBlock> ETERNAL_ANCHOR = BLOCKS.register("eternal_anchor",
            () -> new EternalAnchorBlock(
                    BlockBehaviour.Properties.of()
                            .mapColor(MapColor.COLOR_BLACK)
                            .strength(1.0F)
                            .requiresCorrectToolForDrops()
                            .sound(SoundType.STONE)
                            .lightLevel(s -> 15)));

    public static final DeferredItem<BlockItem> ETERNAL_ANCHOR_ITEM = ITEMS.registerSimpleBlockItem("eternal_anchor",
            ETERNAL_ANCHOR);

    public EternalAnchor(IEventBus modEventBus, ModContainer modContainer) {
        BLOCKS.register(modEventBus);
        ITEMS.register(modEventBus);
        modEventBus.addListener(this::addCreative);
    }

    private void addCreative(BuildCreativeModeTabContentsEvent event) {
        if (event.getTabKey() == CreativeModeTabs.FUNCTIONAL_BLOCKS) {
            event.accept(ETERNAL_ANCHOR_ITEM);
        }
    }
}
