#!/usr/bin/perl
use strict;
local $" = ",";

my $i    = 0;
my @nums = map {
    chomp $_;
    if ( $_ ne "" ) { [ $i++, $_ ] }
    else            { () }
} (<ARGV>);
print "0: ", join( ',', map { sprintf "%3d", $_->[1] } @nums ), "\n" if $ENV{DEBUG} > 1;

my $next = 0;
my $idx  = 0;

while ( $next < @nums ) {

    # print "$idx $next\n";
    my $t = $nums[$idx];

    if ( $t->[0] != $next ) {
        $idx++;
        next;
    }

    my $nidx;
    if ( $t->[1] > 0 ) {
        $nidx = ( $idx + $t->[1] ) % @nums;
        if ( $nidx == $#nums ) {
            $nidx = 0;
        }
    }
    elsif ( $t->[1] < 0 ) {
        $nidx = ( ( $idx + $t->[1] ) % @nums ) - 1;
        if ( $nidx == -1 ) {
            $nidx = $#nums;
        }
    }
    else {
        $idx++;
        $next++;
        next;
    }

    if ( $nidx < $idx ) {
        if ( $idx < $#nums ) {
            @nums = ( @nums[ 0 .. $nidx ], $t, @nums[ $nidx + 1 .. $idx - 1 ], @nums[ $idx + 1 .. $#nums ] );
        }
        else {
            @nums = ( @nums[ 0 .. $nidx ], $t, @nums[ $nidx + 1 .. $idx - 1 ] );
        }
    }
    elsif ( $nidx > $idx ) {
        if ( $idx > 0 ) {
            @nums = ( @nums[ 0 .. $idx - 1 ], @nums[ $idx + 1 .. $nidx ], $t, @nums[ $nidx + 1 .. $#nums ] );
        }
        else {
            @nums = ( @nums[ $idx + 1 .. $nidx ], $t, @nums[ $nidx + 1 .. $#nums ] );
        }
    }
    print "$next: ", join( ',', map { sprintf "%3d", $_->[1] } @nums ), " - moved idx: $idx, ndx: $nidx, dat: $t->[1]]\n"
      if $ENV{DEBUG} > 1;
    $next++;
    $idx = 0;
}

die "didn't find it all" if $next != @nums;

if ( $ENV{DEBUG} ) {
    for my $i ( 0 .. $#nums ) {
        print "$i: @{[$nums[$i]->[1]]}\n";
    }
}

my $zero;
for $idx ( 0 .. $#nums ) {
    if ( $nums[$idx]->[1] == 0 ) {
        $zero = $idx;
    }
}
die "couldn't find zero" if $zero >= @nums;
print "zero at $zero\n";
my @final_idx = map { ( $_ + $zero ) % @nums } ( 1000, 2000, 3000 );
my @output    = map { $nums[$_]->[1] } @final_idx;
print "@final_idx -> @output ", $output[0] + $output[1] + $output[2], "\n";
