package com.eternal_anchor.eternalanchormod;

import java.util.Optional;

import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.sounds.SoundEvents;
import net.minecraft.sounds.SoundSource;
import net.minecraft.world.InteractionResult;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.LevelReader;
import net.minecraft.world.level.block.RespawnAnchorBlock;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.phys.BlockHitResult;

public class EternalAnchorBlock extends RespawnAnchorBlock {

    public EternalAnchorBlock(Properties props) {
        super(props);
        registerDefaultState(stateDefinition.any().setValue(CHARGE, MAX_CHARGES));
    }

    @Override
    protected void onPlace(BlockState state, Level level, BlockPos pos,
                           BlockState oldState, boolean isMoving) {
        super.onPlace(state, level, pos, oldState, isMoving);
        if (!level.isClientSide && state.getValue(CHARGE) < MAX_CHARGES) {
            level.setBlock(pos, state.setValue(CHARGE, MAX_CHARGES), 3);
        }
    }

    // Igual ao vanilla, mas sem a checagem canSetSpawn(level): define o ponto de
    // respawn em qualquer dimensao e nunca explode.
    @Override
    protected InteractionResult useWithoutItem(BlockState state, Level level, BlockPos pos,
                                               Player player, BlockHitResult hitResult) {
        if (state.getValue(CHARGE) == 0) {
            return InteractionResult.PASS;
        }
        if (!level.isClientSide) {
            ServerPlayer serverPlayer = (ServerPlayer) player;
            if (serverPlayer.getRespawnDimension() != level.dimension() || !pos.equals(serverPlayer.getRespawnPosition())) {
                serverPlayer.setRespawnPosition(level.dimension(), pos, 0.0F, false, true);
                level.playSound(null, pos.getX() + 0.5, pos.getY() + 0.5, pos.getZ() + 0.5,
                        SoundEvents.RESPAWN_ANCHOR_SET_SPAWN, SoundSource.BLOCKS, 1.0F, 1.0F);
                return InteractionResult.SUCCESS;
            }
        }
        return InteractionResult.CONSUME;
    }

    // Fallback consultado pelo ServerPlayer quando a checagem vanilla de dimensao
    // falha; sem isto o jogador clicaria mas nao renasceria fora do Nether.
    @Override
    public Optional<ServerPlayer.RespawnPosAngle> getRespawnPosition(BlockState state, EntityType<?> type,
                                                                     LevelReader levelReader, BlockPos pos, float orientation) {
        if (state.getValue(CHARGE) > 0) {
            return findStandUpPosition(type, levelReader, pos)
                    .map(vec -> ServerPlayer.RespawnPosAngle.of(vec, pos));
        }
        return Optional.empty();
    }
}
